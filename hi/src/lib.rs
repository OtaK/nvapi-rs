#![doc(html_root_url = "http://arcnmx.github.io/nvapi-rs/")]

pub extern crate nvapi;

mod display;
pub use self::display::*;
mod gpu;
pub use self::gpu::*;

pub use nvapi::{
    Status, Result,
    sys,
    initialize, unload, driver_version, interface_version, error_message
};

pub fn allowable_result_fallback<T>(v: nvapi::Result<T>, fallback: T) -> nvapi::Result<T> {
    match v {
        Ok(v) => Ok(v),
        Err(Status::NotSupported) | Err(Status::NoImplementation) | Err(Status::ArgumentExceedMaxSize) => Ok(fallback),
        Err(e) => Err(e),
    }
}

pub fn allowable_result<T>(v: nvapi::Result<T>) -> nvapi::Result<nvapi::Result<T>> {
    match v {
        Ok(v) => Ok(Ok(v)),
        Err(e @ Status::NotSupported) | Err(e @ Status::NoImplementation) => Ok(Err(e)),
        Err(e) => Err(e),
    }
}
