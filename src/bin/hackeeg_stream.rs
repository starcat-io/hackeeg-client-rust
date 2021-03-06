// Copyright © 2020 Starcat LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::{info, warn};
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::SerialPortSettings;

use common::constants::ads1299;
use hackeeg::client::commands::responses::Status;
use hackeeg::common::constants::NUM_CHANNELS;
use hackeeg::{client::modes::Mode, client::HackEEGClient, common};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAIN_TAG: &str = "main";
const DEFAULT_STREAM_NAME: &str = "HackEEG";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("HackEEG Streamer")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("port")
                .help("The device path to a serial port")
                .required(true),
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at")
                .default_value("115200")
                .required(true),
        )
        .arg(
            Arg::with_name("sps")
                .short("s")
                .long("--sps")
                .help("Samples per second")
                .default_value("500"),
        )
        .arg(
            Arg::with_name("lsl")
                .short("L")
                .long("lsl")
                .help("Send samples to an LSL stream instead of terminal"),
        )
        .arg(
            Arg::with_name("lsl_stream_name")
                .short("N")
                .long("lsl-stream-name")
                .help("Name of LSL stream to create")
                .default_value(DEFAULT_STREAM_NAME),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Quiet mode: do not print sample data (used for performance testing)"),
        )
        .arg(
            Arg::with_name("samples")
                .short("S")
                .long("samples")
                .help("How many samples to capture")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("messagepack")
                .short("M")
                .long("messagepack")
                .help("MessagePack mode- use MessagePack format to send sample data to the host, rather than JSON Lines")
        )
        .arg(
            Arg::with_name("channel_test")
                .short("T")
                .long("channel-test")
                .help("Set the channels to internal test settings for software testing")
        )
        .arg(
            Arg::with_name("gain")
                .short("g")
                .long("gain")
                .help("ADS1299 gain setting for all channels")
                .default_value("1")
                .takes_value(true)
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbosity") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>()?;
    let sps = matches.value_of("sps").unwrap().parse::<u32>()?;

    common::log::setup_logger(log_level, None)?;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = baud_rate;
    settings.timeout = Duration::from_millis(10);

    let mut client = HackEEGClient::new(port_name, &settings)?;

    client.blink_board_led()?;

    let sample_mode = ads1299::Speed::from(sps) as u8 | ads1299::CONFIG1_const;
    client
        .wreg::<Status>(ads1299::GlobalSettings::CONFIG1 as u8, sample_mode)?
        .assert()?;

    info!(target: MAIN_TAG, "Disabling all channels");
    client.disable_all_channels()?;

    if matches.is_present("channel_test") {
        info!(target: MAIN_TAG, "Enabling channel config test");
        client.channel_config_test()?;
    } else {
        let gain: ads1299::Gain = matches
            .value_of("gain")
            .expect("Expected gain")
            .parse::<u32>()?
            .into();
        info!(target: MAIN_TAG, "Configuring channels with gain {}", gain);
        client.enable_all_channels(Some(gain))?;
    }

    // Route reference electrode to SRB1: JP8:1-2, JP7:NC (not connected)
    // use this with humans to reduce noise
    info!(target: MAIN_TAG, "Enabling reference electrode SRB1");
    client
        .wreg::<Status>(ads1299::MISC1, ads1299::SRB1 | ads1299::MISC1_const)?
        .assert()?;

    // Single-ended mode - setting SRB1 bit sends mid-supply voltage to the N inputs
    // use this with a signal generator
    // client.wreg(ads1299::MISC1, ads1299::SRB1)?;

    // Dual-ended mode
    info!(target: MAIN_TAG, "Setting dual-ended mode");
    client
        .wreg::<Status>(ads1299::MISC1, ads1299::MISC1_const)?
        .assert()?;

    if matches.is_present("messagepack") {
        client.ensure_mode(Mode::MsgPack)?;
    } else {
        client.ensure_mode(Mode::JsonLines)?;
    }
    client.start()?;
    client.rdatac()?;

    let mut maybe_outlet: Option<lsl_sys::Outlet<i32>> = None;

    if matches.is_present("lsl") {
        let stream_name = matches.value_of("lsl_stream_name").unwrap();
        let stream_type = "EEG";
        // derive our uuid from name-type-num_channels
        let uuid_name = format!("{}-{}-{}", stream_name, stream_type, NUM_CHANNELS);
        let stream_id = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, uuid_name.as_bytes())
            .to_simple()
            .to_string();

        let stream_info = lsl_sys::StreamInfo::<i32>::new(
            stream_name,
            stream_type,
            NUM_CHANNELS as i32,
            sps as f64,
            &stream_id,
        )?;
        maybe_outlet = Some(lsl_sys::Outlet::new(stream_info, 0, 360)?);
    }

    let quiet = matches.is_present("quiet");
    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sigint))?;

    let start = std::time::Instant::now();
    let mut counter: u64 = 0;
    let mut errors: u64 = 0;
    let max_samples = match matches.value_of("samples") {
        Some(samples_str) => samples_str.parse::<u64>()?,
        None => 0,
    };

    loop {
        if sigint.load(Ordering::Relaxed) {
            info!(target: MAIN_TAG, "Got SIGINT, breaking read loop");
            break;
        }

        let resp = client.read_rdatac_response();
        match resp {
            Err(e) => {
                errors += 1;
                warn!(target: MAIN_TAG, "Error getting response: {:?}", e);
                continue;
            }
            Ok(sample) => {
                let ch = sample.channels;

                if !quiet {
                    println!(
                        "{} @ {}: [{}, {}, {}, {}, {}, {}, {}, {}]",
                        sample.sample_number,
                        sample.timestamp,
                        ch[0].sample,
                        ch[1].sample,
                        ch[2].sample,
                        ch[3].sample,
                        ch[4].sample,
                        ch[5].sample,
                        ch[6].sample,
                        ch[7].sample
                    );
                }

                if let Some(ref outlet) = maybe_outlet {
                    outlet.push_chunk(sample.as_lsl_data().as_slice(), sample.timestamp as f64);
                }

                counter += 1;

                if max_samples > 0 && counter >= max_samples {
                    info!(
                        target: MAIN_TAG,
                        "Reached {} samples, breaking", max_samples
                    );
                    break;
                }
            }
        }
    }

    let elapsed = start.elapsed();
    info!(
        target: MAIN_TAG,
        "{} samples ({} errors) in {} seconds, or {}/s",
        counter,
        errors,
        elapsed.as_secs_f32(),
        counter as f32 / elapsed.as_secs_f32()
    );

    Ok(())
}
