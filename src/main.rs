use log::info;
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use serialport::prelude::SerialPortSettings;

use hackeeg::{client::HackEEGClient, common};

const MAIN_TAG: &str = "main";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Serialport Example - Receive Data")
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
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("baud")
                .help("The baud rate to connect at")
                .use_delimiter(false)
                .default_value("115200")
                .required(true),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbosity") {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>()?;

    common::log::setup_logger(log_level, None)?;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = baud_rate;
    settings.timeout = Duration::from_millis(10);

    info!(
        target: MAIN_TAG,
        "Creating client connection to {}", port_name
    );
    let client = HackEEGClient::new(port_name, &settings)?;
    client.blink_test(10)?;

    Ok(())
}
