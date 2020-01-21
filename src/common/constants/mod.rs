pub mod ads1299;

pub const NUM_CHANNELS: usize = 8;

// message pack manual sizes and offsets, for faster decoding
pub const MP_MESSAGE_SIZE: usize = 44;
pub const MP_BINARY_OFFSET: usize = 9;
