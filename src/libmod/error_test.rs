#[cfg(test)] mod tests {
  use crate::libmod::CursorSizeErr;
  use super::*;

  #[test] fn format_error() {
    assert_eq!(format!("{}",CursorSizeErr::DXDupe("?u?".to_string())),"DX Duplication error: ?u?");
  }
}
