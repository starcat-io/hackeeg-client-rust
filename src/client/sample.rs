use std::convert::TryInto;

const NUM_CHANNELS: usize = 8;

#[derive(Copy, Clone)]
pub struct Channel {
    sample: i32,
}

impl From<&[u8]> for Channel {
    fn from(data: &[u8]) -> Self {
        // each sample data is signed 24 bit
        let sample = i32::from_be_bytes(data[0..4].try_into().unwrap()) >> 1;
        Self { sample }
    }
}

pub struct Sample {
    timestamp: chrono::DateTime<chrono::Utc>,
    sample_number: u32,
    ads_status: u32,
    ads_gpio: u8,
    loff_statn: u8,
    loff_statp: u8,
    extra: u8,
    channels: [Channel; NUM_CHANNELS],
}

impl From<&[u8]> for Sample {
    fn from(data: &[u8]) -> Self {
        let timestamp_secs = u32::from_le_bytes(data[0..4].try_into().unwrap()) as i64;
        let naive_dt = chrono::NaiveDateTime::from_timestamp(timestamp_secs, 0);
        let utc_dt = chrono::DateTime::from_utc(naive_dt, chrono::Utc);

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
            timestamp: utc_dt,
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
