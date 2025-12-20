use crate::libmod::cur_box;

use thiserror::Error;

#[derive(Debug,Error)]
pub enum CursorSizeErr {
  #[error("DX Duplication error: {0}")]
  DXDupe(String),
  #[error("Failed parsing cursor bitmap: {0}")]
  Bitmap(String),
  #[error("IconInfo error: {0}")]
  Ii(String),
  #[error("Bounding box has invalid size: {0}")]
  BoxSzInvalid(cur_box),
  #[error("🛑GDI resource leak!")]
  ResourceLeak(String),
}

/*
use std::num::NonZeroU8;
#[repr(transparent)] //allows returning in FFI since its layout = u8
pub struct ErrC(NonZeroU8);

// const TEN: NonZeroU8 = NonZeroU8::new(10).expect("ten is non-zero");
const DXDupe      	: ErrC = ErrC(NonZeroU8::new(1).expect("1 ≠ 0"));
const Bitmap      	: ErrC = ErrC(NonZeroU8::new(2).expect("2 ≠ 0"));
const Ii          	: ErrC = ErrC(NonZeroU8::new(3).expect("3 ≠ 0"));
const BoxSzInvalid	: ErrC = ErrC(NonZeroU8::new(4).expect("4 ≠ 0"));
const ResourceLeak	: ErrC = ErrC(NonZeroU8::new(5).expect("5 ≠ 0"));

#[unsafe(no_mangle)] pub extern "C"
fn test_error_code1() -> Option<ErrC> {
  Some(DXDupe)
}
#[unsafe(no_mangle)] pub extern "C"
fn test_error_code0() -> Option<ErrC> {
  None
}
*/
