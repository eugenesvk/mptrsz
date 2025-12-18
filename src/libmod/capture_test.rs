#[cfg(test)] mod tests {
  use crate::*;
  use rusty_duplication::Scanner;
  use crate::libmod::*;

  #[test] fn cursor_color_debug_display() {
    let b = CursorColor::Mono;
    let c = CursorColor::Colorμ;
    let m = CursorColor::Colorα;
    pp!("
      \n{b}\n{b:#}\n{b:?}\n{b:#?}
      \n{c}\n{c:#}\n{c:?}\n{c:#?}
      \n{m}\n{m:#}\n{m:?}\n{m:#?}",
    );
  }
}
