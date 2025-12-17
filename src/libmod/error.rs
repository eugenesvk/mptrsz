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
