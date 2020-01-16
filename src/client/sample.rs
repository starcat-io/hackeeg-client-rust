use crate::common::constants::NUM_CHANNELS;
use byteorder::ByteOrder;
use std::convert::TryInto;

#[derive(Copy, Clone)]
pub struct Channel {
    pub sample: i32,
}

impl From<&[u8]> for Channel {
    fn from(data: &[u8]) -> Self {
        let sample = byteorder::BigEndian::read_i24(data);
        Self { sample }
    }
}

pub struct Payload {
    pub timestamp: u32,
    pub sample_number: u32,
    pub ads_status: u32,
    pub ads_gpio: u8,
    pub loff_statn: u8,
    pub loff_statp: u8,
    pub extra: u8,
    pub channels: [Channel; NUM_CHANNELS],
}

impl From<&[u8]> for Payload {
    fn from(data: &[u8]) -> Self {
        let timestamp = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let sample_number = u32::from_le_bytes(data[4..8].try_into().unwrap());

        // we shift by 1 because ads_status is technically 3 big endian bytes, but from_be_bytes
        // can only take [u8; 4]
        let ads_status = u32::from_be_bytes(data[8..12].try_into().unwrap()) >> 1;

        let ads_gpio = (ads_status & 0x0f) as u8;
        let loff_statn = ((ads_status >> 4) & 0xff) as u8;
        let loff_statp = ((ads_status >> 12) & 0xff) as u8;
        let extra = ((ads_status >> 20) & 0xff) as u8;

        let chan_offset = 11;
        let mut channels = [Channel { sample: 0 }; NUM_CHANNELS];
        for chan_idx in 0..NUM_CHANNELS {
            let chan_start = chan_offset + (chan_idx * 3);
            channels[chan_idx] = data[chan_start..chan_start + 3].into();
        }

        Self {
            timestamp,
            sample_number,
            ads_status,
            ads_gpio,
            loff_statn,
            loff_statp,
            extra,
            channels,
        }
    }
}
