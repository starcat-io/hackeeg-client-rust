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

use log::{debug, info, trace, warn};
use lsl_sys;
use serde_json::json;
use serialport::prelude::*;
use serialport::Result as SerialResult;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Read};
use std::time::Duration;

pub mod commands;
mod err;
pub mod modes;
mod sample;

use crate::client::commands::responses::Status;
use crate::common::constants;
use commands::args::NoArgs;
use constants::ads1299;
use err::ClientError;
use modes::Mode;
use std::ops::Deref;

const CLIENT_TAG: &str = "hackeeg_client";

pub struct HackEEGClient {
    port_name: String,
    port: RefCell<BufReader<Box<dyn SerialPort>>>,
    mode: Mode,
    continuous_read: Cell<bool>,
}

type ClientResult<T> = Result<T, err::ClientError>;

impl HackEEGClient {
    pub fn new(port_name: &str, settings: &SerialPortSettings) -> Result<Self, Box<dyn Error>> {
        info!(
            target: CLIENT_TAG,
            "Creating client connection to {}", port_name
        );
        let port = serialport::open_with_settings(port_name, settings)?;

        // construct our client
        let mut client = Self {
            port_name: port_name.to_string(),
            port: RefCell::new(BufReader::new(port)),
            mode: Mode::Unknown,
            continuous_read: Cell::new(false),
        };

        client.ensure_mode(Mode::JsonLines)?;

        Ok(client)
    }

    pub fn enable_all_channels(&self, gain: Option<ads1299::Gain>) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Enabling all channels");
        for chan_idx in 1..=constants::NUM_CHANNELS {
            self.enable_channel(chan_idx as u8, gain)?
        }
        Ok(())
    }

    pub fn enable_channel(&self, chan_num: u8, gain: Option<ads1299::Gain>) -> ClientResult<()> {
        let gain = gain.unwrap_or(constants::ads1299::Gain::X1);

        info!(
            target: CLIENT_TAG,
            "Enabling channel {} with gain {}", chan_num, gain
        );

        let was_reading = if self.continuous_read.get() {
            debug!(
                target: CLIENT_TAG,
                "We're in continuous read mode, temporarily disabling"
            );
            self.sdatac()?;
            true
        } else {
            false
        };

        let status: Status = self.wreg(
            ads1299::ChannelSettings::CHnSET as u8 + chan_num,
            ads1299::ELECTRODE_INPUT | gain as u8,
        )?;
        status.assert();

        if was_reading {
            debug!(
                target: CLIENT_TAG,
                "We were in continuous read, re-enabling"
            );
            self.rdatac()?;
        }

        Ok(())
    }

    pub fn channel_config_test(&self) -> ClientResult<()> {
        let map_status = |status: Status| status.assert();

        self.wreg(
            ads1299::GlobalSettings::CONFIG2 as u8,
            ads1299::INT_TEST_4HZ | ads1299::CONFIG2_const,
        )
        .map(map_status)?;

        self.wreg(
            ads1299::ChannelSettings::CH1SET as u8,
            ads1299::INT_TEST_DC | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;

        self.wreg(
            ads1299::ChannelSettings::CH2SET as u8,
            ads1299::SHORTED | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;

        self.wreg(
            ads1299::ChannelSettings::CH3SET as u8,
            ads1299::MVDD | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;
        self.wreg(
            ads1299::ChannelSettings::CH4SET as u8,
            ads1299::BIAS_DRN | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;
        self.wreg(
            ads1299::ChannelSettings::CH5SET as u8,
            ads1299::BIAS_DRP | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;
        self.wreg(
            ads1299::ChannelSettings::CH6SET as u8,
            ads1299::TEMP | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;
        self.wreg(
            ads1299::ChannelSettings::CH7SET as u8,
            ads1299::TEST_SIGNAL | ads1299::Gain::X1 as u8,
        )
        .map(map_status)?;

        self.disable_channel(8)?;
        Ok(())
    }

    pub fn wreg<T>(&self, reg: u8, val: u8) -> ClientResult<T>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        debug!(target: CLIENT_TAG, "Writing {} to register {}", val, reg);
        self.execute_json_cmd("wreg", [reg, val])
    }

    pub fn disable_all_channels(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Disabling all channels");
        for chan_idx in 1..=constants::NUM_CHANNELS {
            self.disable_channel(chan_idx as u8)?;
        }
        Ok(())
    }

    pub fn disable_channel(&self, chan_num: u8) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Disabling channel {}", chan_num);

        let status: Status = self.wreg(
            ads1299::ChannelSettings::CHnSET as u8 + chan_num,
            ads1299::PDn | ads1299::SHORTED,
        )?;
        status.assert()?;
        Ok(())
    }

    pub fn blink_test(&self, num: u32) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Starting blink test.");
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

    pub fn noop(&self) -> ClientResult<bool> {
        // no-op is a little special in that it can be expected to fail on deserialization, and
        // that isn't considered an error
        match self.execute_json_cmd("nop", NoArgs) {
            Ok(Status {
                status_code,
                status_text,
            }) => Ok(true),
            Err(ClientError::DeserializeError(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn board_led_on(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Turning board LED on");
        let status: Status = self.execute_json_cmd("boardledon", NoArgs)?;
        status.assert()?;
        Ok(())
    }

    pub fn board_led_off(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Turning board LED off");
        let status: Status = self.execute_json_cmd("boardledoff", NoArgs)?;
        status.assert()?;
        Ok(())
    }

    pub fn blink_board_led(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "Blinking board LED");
        self.board_led_on()?;
        std::thread::sleep(std::time::Duration::from_millis(300));
        self.board_led_off()?;
        Ok(())
    }

    pub fn send_text_cmd(&self, cmd: &str) -> IOResult<()> {
        debug!(target: CLIENT_TAG, "Sending text command '{}'", cmd);
        let mut port = self.port.borrow_mut();
        let mut full_cmd = cmd.to_string();
        full_cmd.push('\n');
        port.get_mut().write(full_cmd.as_bytes())?;

        drop(port);
        self.read_response_line()?;
        Ok(())
    }

    fn read_response_line(&self) -> IOResult<String> {
        let mut port = self.port.borrow_mut();
        let mut buf = String::new();
        port.read_line(&mut buf);
        Ok(buf)
    }

    /// Executes a json command and deserializes the result as `T`.  Since `T` has
    /// `DeserializeOwned`, this performs a copy.  For very high performance, write another function
    /// that passes in the buffer and bounds `T` with `Deserialize<'de>` instead, for no copies.
    pub fn execute_json_cmd<T, G>(&self, cmd: &str, args: G) -> ClientResult<T>
    where
        T: serde::de::DeserializeOwned + Clone,
        G: serde::Serialize,
    {
        debug!(
            target: CLIENT_TAG,
            "Executing JSON command '{}' and then reading response", cmd
        );

        let to_send = json_cmd_line(cmd, args);
        self.port.borrow_mut().get_mut().write(to_send.as_bytes())?;

        let mut buf = vec![0; 1024];
        let resp = self.read_response_line()?;
        trace!(target: CLIENT_TAG, "Got response: {}", resp.trim());

        Ok(serde_json::from_str(&resp)?)
    }

    // stop data continuous
    pub fn sdatac(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "sdatac");
        let status: Status = self.execute_json_cmd("sdatac", NoArgs)?;
        status.assert()?;
        self.continuous_read.set(false);
        Ok(())
    }

    // read data continuous
    pub fn rdatac(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "rdatac");
        let status: Status = self.execute_json_cmd("rdatac", NoArgs)?;
        status.assert()?;
        self.continuous_read.set(true);
        Ok(())
    }

    pub fn start(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "start");
        let status: Status = self.execute_json_cmd("start", NoArgs)?;
        status.assert()?;
        Ok(())
    }

    pub fn stop(&self) -> ClientResult<()> {
        info!(target: CLIENT_TAG, "stop");
        let status: Status = self.execute_json_cmd("stop", NoArgs)?;
        status.assert()?;
        Ok(())
    }

    fn messagepack_read(&self) -> ClientResult<sample::Sample> {
        let mut port = self.port.borrow_mut();
        let mut mp_buf = [0; constants::MP_MESSAGE_SIZE];
        port.read_exact(&mut mp_buf)?;
        let sample = mp_buf[constants::MP_BINARY_OFFSET..].into();
        Ok(sample)
    }

    pub fn read_rdatac_response(&self) -> ClientResult<sample::Sample> {
        if self.mode == Mode::MsgPack {
            let sample = self.messagepack_read()?;
            Ok(sample)
        } else {
            let resp = self.read_response_line()?;

            trace!(target: CLIENT_TAG, "Raw rdatac response line: {:?}", resp);
            let payload: commands::responses::JSONPayload = serde_json::from_str(&resp)?;
            Ok(payload.data.into())
        }
    }

    pub fn stop_and_sdatac_messagepack(&self) -> ClientResult<()> {
        self.stop()?;
        self.sdatac()?;
        self.noop()?;

        // we have to drain until EOF on the port.  i'm not totally sure why
        self.drain_to_eof()?;
        Ok(())
    }

    pub fn drain_to_eof(&self) -> ClientResult<usize> {
        debug!(target: CLIENT_TAG, "Draining port to EOF...");
        let mut port = self.port.borrow_mut();
        let mut buf = vec![];

        match port.read_to_end(&mut buf) {
            Ok(amt) => {
                debug!(target: CLIENT_TAG, "Drained {} bytes", amt);
                Ok(amt)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                warn!(target: CLIENT_TAG, "Timed out draining, but that's ok");
                Ok(0)
            }
            Err(e) => Err(ClientError::from(e)),
        }
    }

    /// Ensures that the device is in the desired mode, and returns whether it had to change it
    /// into that mode in order to ensure
    pub fn ensure_mode(&mut self, desired_mode: Mode) -> ClientResult<bool> {
        info!(
            target: CLIENT_TAG,
            "Ensuring we're in mode {:?}", desired_mode
        );
        if self.mode != desired_mode {
            debug!(
                target: CLIENT_TAG,
                "Desired mode {:?} doesn't match current mode {:?}", desired_mode, self.mode
            );

            match desired_mode {
                Mode::Text => match self.mode {
                    Mode::JsonLines => {
                        self.send_text_cmd("jsonlines")?;
                    }
                    Mode::MsgPack => {
                        self.send_text_cmd("jsonlines")?;
                        self.send_text_cmd("messagepack")?;
                    }
                    _ => unreachable!(),
                },
                Mode::JsonLines => match self.mode {
                    Mode::MsgPack => {
                        self.send_text_cmd("jsonlines")?;
                    }
                    Mode::Text | Mode::Unknown => {
                        self.stop();
                        // notice we're ignoring the potential error result here.  if we're not
                        // in jsonlines mode already, sdatac will fail
                        self.sdatac();
                        self.drain_to_eof()?;
                        self.send_text_cmd("jsonlines")?;
                        self.noop()?;
                    }
                    _ => unreachable!(),
                },
                Mode::MsgPack => match self.mode {
                    Mode::JsonLines => {
                        let status: Status = self.execute_json_cmd("messagepack", NoArgs)?;
                        status.assert()?;
                    }
                    Mode::Text => {
                        self.send_text_cmd("jsonlines")?;
                        let status: Status = self.execute_json_cmd("messagepack", NoArgs)?;
                        status.assert()?;
                    }
                    _ => unreachable!(),
                },
                // we should never get here, because our new() method determines the current mode
                Mode::Unknown => unreachable!(),
            }

            self.mode = desired_mode;
            Ok(true)
        } else {
            debug!(target: CLIENT_TAG, "We're already in mode {:?}", self.mode);
            Ok(false)
        }
    }
}

fn json_cmd<G>(cmd: &str, args: G) -> String
where
    G: serde::Serialize,
{
    let params = serde_json::to_value(&args).unwrap();
    if params.is_null() {
        json!({
            "COMMAND": cmd,
            "PARAMETERS": [],
        })
        .to_string()
    } else {
        json!({
            "COMMAND": cmd,
            "PARAMETERS": params,
        })
        .to_string()
    }
}

fn json_cmd_line<G>(cmd: &str, args: G) -> String
where
    G: serde::Serialize,
{
    json_cmd(cmd, args) + "\r\n"
}
