use std::alloc::handle_alloc_error;
use std::ffi;
use std::ffi::NulError;
use std::fmt::Formatter;
use std::marker::PhantomData;

#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code
)]
mod bindings;

#[derive(Debug)]
pub enum Error {
    StreamConstructionErr,
    OutletConstructionErr,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        unimplemented!()
    }
}

type Result<T> = std::result::Result<T, Error>;

impl From<ffi::NulError> for Error {
    fn from(_: NulError) -> Self {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
enum ChannelFormat {
    Undefined = 0,
    Float32,
    Double64,
    String,
    Int32,
    Int16,
    Int8,
    Int64,
}

pub struct StreamInfo<Format> {
    handle: bindings::lsl_streaminfo,
    phantom: PhantomData<Format>,
}

impl<Format> StreamInfo<Format> {
    fn real_new(
        name: &str,
        stream_type: &str,
        channel_count: i32,
        nominal_srate: f64,
        source_id: &str,
        channel_format: ChannelFormat,
    ) -> Result<Self> {
        let name_cstring = ffi::CString::new(name)?;
        let stream_cstring = ffi::CString::new(stream_type)?;

        unsafe {
            let handle = bindings::lsl_create_streaminfo(
                ffi::CString::new(name)?.into_raw(),
                ffi::CString::new(stream_type)?.into_raw(),
                channel_count,
                nominal_srate,
                channel_format as bindings::lsl_channel_format_t,
                ffi::CString::new(source_id)?.into_raw(),
            );
            if handle.is_null() {
                Err(Error::StreamConstructionErr)
            } else {
                Ok(Self {
                    handle,
                    phantom: PhantomData,
                })
            }
        }
    }
}

impl StreamInfo<i32> {
    pub fn new(
        name: &str,
        stream_type: &str,
        channel_count: i32,
        nominal_srate: f64,
        source_id: &str,
    ) -> Result<Self> {
        StreamInfo::real_new(
            name,
            stream_type,
            channel_count,
            nominal_srate,
            source_id,
            ChannelFormat::Int32,
        )
    }
}

pub struct Outlet<Format> {
    info: StreamInfo<Format>,
    handle: bindings::lsl_outlet,
}

impl Outlet<i32> {
    pub fn push_chunk(&self, data: &[i32], timestamp: f64) -> i32 {
        unsafe {
            bindings::lsl_push_chunk_it(self.handle, data.as_ptr(), data.len() as u64, timestamp)
        }
    }
}

impl Outlet<f32> {
    pub fn push_chunk(&self, data: &[f32], timestamp: f64) -> i32 {
        unsafe {
            bindings::lsl_push_chunk_ft(self.handle, data.as_ptr(), data.len() as u64, timestamp)
        }
    }
}

impl<Format> Outlet<Format> {
    pub fn new(info: StreamInfo<Format>, chunk_size: i32, max_buffered: i32) -> Result<Self> {
        unsafe {
            let handle = bindings::lsl_create_outlet(info.handle, chunk_size, max_buffered);
            if handle.is_null() {
                Err(Error::OutletConstructionErr)
            } else {
                Ok(Self { info, handle })
            }
        }
    }
}

impl<Format> Drop for Outlet<Format> {
    fn drop(&mut self) {
        unsafe {
            bindings::lsl_destroy_outlet(self.handle);
        }
    }
}
