extern crate clap;
extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;
use std::{thread, time};

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;

use serde_json::{json};

const BLINK_PAUSE_IN_MILLISECONDS : u64 = 200;

fn get_command(command: String) -> String {
    let command_value = json!({
                    "COMMAND": command,
                    "PARAMETERS": [],
                });
    return command_value.to_string();
}

fn get_command_line(command: String) -> String {
    let command: (String) = get_command(command);
    let command_line: (String) = command.to_string() +  "\r\n";
    return command_line;
}

fn send_command_line(port: &mut Box<dyn SerialPort>, command: String) {
    let command_line = get_command_line(command);
    match port.write(command_line.as_bytes()) {
        Ok(_) => {
            println!("command sent:");
            println!("{}", command_line);
            std::io::stdout().flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn read_response(port: &mut Box<dyn SerialPort>) {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn execute_command(port: &mut Box<dyn SerialPort>, command: String) {
    send_command_line(port, command);
    read_response(port);
}


fn main() {
let matches = App::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .setting(AppSettings::DisableVersion)
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
                .required(true),
        )
        .get_matches();
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap();

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        eprintln!("Error: Invalid baud rate '{}' specified", baud_rate);
        ::std::process::exit(1);
    }

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

            // switch to jsonlines mode
            let jsonlines_command = "jsonlines".to_string();
            send_command_line(&mut port, jsonlines_command);

            let nop_command: (String) = get_command("nop".to_string());
            execute_command(&mut port, nop_command);

            let blink_pause_duration= time::Duration::from_millis(BLINK_PAUSE_IN_MILLISECONDS);
            loop {
                execute_command(&mut port, "boardledon".to_string());
                thread::sleep(blink_pause_duration);
                execute_command(&mut port, "boardledoff".to_string());
                thread::sleep(blink_pause_duration);
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

