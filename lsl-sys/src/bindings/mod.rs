use crate::bindings::lsl_bindings::lsl_channel_format_t;
use std::alloc::handle_alloc_error;
use std::ffi;
use std::ffi::NulError;

#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code
)]
mod lsl_bindings;

pub struct Error {}
type Result<T> = std::result::Result<T, Error>;

impl From<ffi::NulError> for Error {
    fn from(_: NulError) -> Self {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
pub enum ChannelFormat {
    Undefined = 0,
    Float32,
    Double64,
    String,
    Int32,
    Int16,
    Int8,
    Int64,
}

pub struct StreamInfo {
    handle: lsl_bindings::lsl_streaminfo,
}

impl StreamInfo {
    pub fn new(
        name: &str,
        stream_type: &str,
        channel_count: i32,
        nominal_srate: f64,
        channel_format: &ChannelFormat,
        source_id: &str,
    ) -> Result<Self> {
        let name_cstring = ffi::CString::new(name)?;
        let stream_cstring = ffi::CString::new(stream_type)?;

        let handle;
        unsafe {
            handle = lsl_bindings::lsl_create_streaminfo(
                ffi::CString::new(name)?.into_raw(),
                ffi::CString::new(stream_type)?.into_raw(),
                channel_count,
                nominal_srate,
                *channel_format as lsl_channel_format_t,
                ffi::CString::new(source_id)?.into_raw(),
            );
        }
        Ok(StreamInfo { handle })
    }
}

pub struct Outlet {
    info: StreamInfo,
}

impl Outlet {
    pub fn new(info: StreamInfo) -> Self {
        Self { info }
    }

    pub fn push_chunk(&self) {
        unsafe {}
    }
}
//pub fn lsl_push_chunk_it(
//    out: lsl_outlet,
//    data: *const i32,
//    data_elements: ::std::os::raw::c_ulong,
//    timestamp: f64,
//) -> i32;

//pub fn lsl_create_outlet(
//    info: lsl_streaminfo,
//    chunk_size: i32,
//    max_buffered: i32,
//) -> lsl_outlet;

//pub fn lsl_destroy_outlet(out: lsl_outlet);
