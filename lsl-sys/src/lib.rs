use std::alloc::handle_alloc_error;
use std::ffi;
use std::ffi::NulError;

#[allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    dead_code
)]
mod bindings;

pub enum Error {
    StreamConstructionErr,
    OutletConstructionErr,
}
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
    handle: bindings::lsl_streaminfo,
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

        unsafe {
            let handle = bindings::lsl_create_streaminfo(
                ffi::CString::new(name)?.into_raw(),
                ffi::CString::new(stream_type)?.into_raw(),
                channel_count,
                nominal_srate,
                *channel_format as bindings::lsl_channel_format_t,
                ffi::CString::new(source_id)?.into_raw(),
            );
            if handle.is_null() {
                Err(Error::StreamConstructionErr)
            } else {
                Ok(Self { handle })
            }
        }
    }
}

trait PushOutlet<Push> {
    fn push_chunk(&self, data: &[Push], num_elements: u64, timestamp: f64) -> i32;
}

pub struct Outlet {
    info: StreamInfo,
    handle: bindings::lsl_outlet,
}

impl PushOutlet<i32> for Outlet {
    fn push_chunk(&self, data: &[i32], num_elements: u64, timestamp: f64) -> i32 {
        unsafe { bindings::lsl_push_chunk_it(self.handle, data.as_ptr(), num_elements, timestamp) }
    }
}

impl PushOutlet<f32> for Outlet {
    fn push_chunk(&self, data: &[f32], num_elements: u64, timestamp: f64) -> i32 {
        unsafe { bindings::lsl_push_chunk_ft(self.handle, data.as_ptr(), num_elements, timestamp) }
    }
}

impl Outlet {
    pub fn new(info: StreamInfo, chunk_size: i32, max_buffered: i32) -> Result<Self> {
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

impl Drop for Outlet {
    fn drop(&mut self) {
        unsafe {
            bindings::lsl_destroy_outlet(self.handle);
        }
    }
}
