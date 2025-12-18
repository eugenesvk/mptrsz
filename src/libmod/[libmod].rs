pub mod ffi_api;

mod error;
mod common;
mod capture;
mod measure;
mod get_cursor_sz;

mod error_test;
mod capture_test;

pub use error::*;
pub use common::*;
pub use capture::*;
pub use measure::*;
pub use get_cursor_sz::*;
pub use ffi_api::*;

pub fn ret42() -> i32 { 42 }
