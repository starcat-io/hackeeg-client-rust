use log::info;
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

    info!(target: MAIN_TAG, "Enabling channel config test");
    client.channel_config_test()?;

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

    loop {
        let resp = client.read_rdatac_response()?;
        let ch = resp.channels;

        if !quiet {
            println!(
                "{} @ {}: [{}, {}, {}, {}, {}, {}, {}, {}]",
                resp.sample_number,
                resp.timestamp,
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
            outlet.push_chunk(resp.as_lsl_data().as_slice(), resp.timestamp as f64);
        }

        counter += 1;

        if sigint.load(Ordering::Relaxed) {
            break;
        }
    }

    let elapsed = start.elapsed();
    println!(
        "\n{} samples in {} seconds, or {}/s",
        counter,
        elapsed.as_secs_f32(),
        counter as f32 / elapsed.as_secs_f32()
    );

    Ok(())
}
