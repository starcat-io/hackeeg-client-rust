use log::{debug, info, trace};
use serde_json::json;
use serialport::prelude::*;
use serialport::Result as SerialResult;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::io::Result as IOResult;
use std::time::Duration;

mod err;
mod modes;
use crate::client::err::ClientError;
use modes::Mode;

mod commands;

const CLIENT_TAG: &str = "hackeeg_client";

pub struct HackEEGClient {
    port_name: String,
    port: RefCell<Box<dyn SerialPort>>,
    mode: Cell<Mode>, // TODO maybe make this not a Cell, think about it more
}

type ClientResult<T> = Result<T, err::ClientError>;

impl HackEEGClient {
    pub fn new(port_name: &str, settings: &SerialPortSettings) -> Result<Self, Box<dyn Error>> {
        let port = serialport::open_with_settings(port_name, settings)?;

        // construct our client
        let mut client = Self {
            port_name: port_name.to_string(),
            port: RefCell::new(port),
            mode: Cell::new(Mode::Unknown),
        };

        // detect our client mode
        let detected_mode = client.sense_mode()?;
        client.mode.set(detected_mode);

        Ok(client)
    }

    /// Determines what mode we're in.  Currently only called at client initialization
    fn sense_mode(&self) -> ClientResult<Mode> {
        // TODO make this real

        //        self.send_json_cmd("stop")?;
        //        self.send_json_cmd("sdatac")?;
        //        match self.execute_json_cmd("nop") {}
        Ok(Mode::Text)
    }

    pub fn jsonlines(&self) -> IOResult<usize> {
        self.send_text_cmd("jsonlines")
    }

    /// Ensures that the device is in the desired mode, and returns whether it had to change it
    /// into that mode in order to ensure
    fn ensure_mode(&self, desired_mode: Mode) -> IOResult<bool> {
        let cur_mode = self.mode.get();

        if cur_mode != desired_mode {
            debug!(
                target: CLIENT_TAG,
                "Desired mode {:?} doesn't match current mode {:?}", desired_mode, cur_mode
            );

            let mut port = self.port.borrow_mut();

            // FIXME i'm not sure this matrix is correct.  for example, i don't know how to go
            // to Mode::Text
            match desired_mode {
                Mode::Text => match cur_mode {
                    Mode::JsonLines => {
                        port.write("jsonlines".as_bytes())?;
                    }
                    Mode::MsgPack => {
                        port.write("jsonlines".as_bytes())?;
                        port.write(json_cmd_line("messagepack").as_bytes())?;
                    }
                    _ => unreachable!(),
                },
                Mode::JsonLines => match cur_mode {
                    Mode::MsgPack => {
                        port.write(json_cmd_line("messagepack").as_bytes())?;
                    }
                    // FIXME is this correct?
                    Mode::Text => {
                        port.write(json_cmd_line("stop").as_bytes())?;
                    }
                    _ => unreachable!(),
                },
                Mode::MsgPack => match cur_mode {
                    Mode::JsonLines => {
                        port.write("jsonlines".as_bytes())?;
                    }
                    // FIXME is this correct?
                    Mode::Text => {
                        port.write(json_cmd_line("stop").as_bytes())?;
                    }
                    _ => unreachable!(),
                },
                // we should never get here, because our new() method determines the current mode
                Mode::Unknown => unreachable!(),
            }

            self.mode.set(desired_mode);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn blink_test(&self, num: u32) -> IOResult<()> {
        info!("Starting blink test.");
        let sleep = || std::thread::sleep(std::time::Duration::from_millis(100));
        for i in 0..num {
            info!("Blinking {} more times", num - i);
            self.board_led_on()?;
            sleep();
            self.board_led_off()?;
            sleep();
        }
        Ok(())
    }

    pub fn board_led_on(&self) -> IOResult<()> {
        info!(target: CLIENT_TAG, "Turning board LED on");
        self.send_json_cmd("boardledon")?;
        Ok(())
    }

    pub fn board_led_off(&self) -> IOResult<()> {
        info!(target: CLIENT_TAG, "Turning board LED off");
        self.send_json_cmd("boardledoff")?;
        Ok(())
    }

    pub fn send_json_cmd(&self, cmd: &str) -> IOResult<usize> {
        debug!(target: CLIENT_TAG, "Sending JSON command {}", cmd);
        self.ensure_mode(Mode::JsonLines)?;
        self.port.borrow_mut().write(json_cmd_line(cmd).as_bytes())
    }

    pub fn send_text_cmd(&self, cmd: &str) -> IOResult<usize> {
        debug!(target: CLIENT_TAG, "Sending text command {}", cmd);
        self.ensure_mode(Mode::Text)?;
        self.port.borrow_mut().write(cmd.as_bytes())
    }

    fn read_response(&self, buffer: &mut Vec<u8>) -> IOResult<usize> {
        // FIXME what happens if we fill the buffer?  might need to exponentially grow the buffer
        // to continue to read the rest of the data.  for now though, we just assume the buffer is
        // big enough
        let _read = self.port.borrow_mut().read(buffer.as_mut_slice())?;
        Ok(_read)
    }

    /// Executes a json command and deserializes the result as `T`.  Since `T` has
    /// `DeserializeOwned`, this performs a copy.  For very high performance, write another function
    /// that passes in the buffer and bounds `T` with `Deserialize<'de>` instead, for no copies.
    pub fn execute_json_cmd<T>(&self, cmd: &str) -> ClientResult<T>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        self.send_json_cmd(cmd)?;

        let mut buf = vec![0; 1024];
        let _read = self.read_response(&mut buf)?;

        Ok(serde_json::from_slice(buf.as_slice())?)
    }
}

fn json_cmd(cmd: &str) -> String {
    let value = json!({
        "COMMAND": cmd,
        "PARAMETERS": [],
    });
    return value.to_string();
}

fn json_cmd_line(cmd: &str) -> String {
    json_cmd(cmd) + "\r\n"
}
