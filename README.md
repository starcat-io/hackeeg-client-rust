# hackeeg-rust-client
Rust client software for [HackEEG TI ADS1299 Arduino shield](https://github.com/starcat-io/hackeeg-shield)

## Rust Client Software

The Rust client software is designed to run on a laptop computer or embedded Linux computer like a Raspberry Pi. This repo provides a `hackeeg_stream` program for streaming data via [Lab Streaming Layer](https://github.com/sccn/labstreaminglayer). 

The `hackeeg_stream` program set the Arduino driver to JSON Lines mode, and communicate with it that way. They issue JSON Lines commands to the Arduino, and recieve JSON Lines or MessagePack data in response.

On a Raspberry Pi 4, connected to an Arduino Due configured to use the SPI DMA included in the driver, and using the MessagePack mode, the `hackeeg_stream` program can read and transfer 8 channels of 24-bit resolution data at 16,384 samples per second, the maximum rate of the ADS1299 chip.

## Notes

This software is only known to work on Linux. MacOS and Windows are not supported.

## Credits

Thanks to Andrew Moffat who is largely responsible for writing this software!

## Contact

If you have questions or comments, please get in touch!

Adam Feuer<br>
adam@starcat.io
