use std::str::FromStr;
use bpaf::*;

// #[derive(Debug,Clone)] pub struct Opt {pub fmt:usize, pub log:bool, pub rows:Vec<u16>}
#[derive(Debug,Clone)] pub struct Opt {pub p_ci:bool, pub p_dx:bool, pub coord:bool, pub shadow:bool, pub rows:Vec<usize>,}

// use owo_colors::OwoColorize;
pub fn options() -> OptionParser<Opt> {
  let rows	= pos::<String>("ROWS").h({let mut d = Doc::default();
    d.text("Print Binary mask (0¦1) or Color/color mask (255 0 0 255) values for these rows\n ");
    d.lit("‘1 2 3’");d.text(" space-separated list\n ");
    d.lit("‘1,2,3’");d.text(" comma-separated list");
    d})
    .parse(|s| {s.split(',').map(usize::from_str).collect::<Result<Vec<_>,_>>()})
    .many().map(|nested| nested.into_iter().flatten().collect());

  // let fmt	= s('f').l("fmt"   ).h({let mut d = Doc::default();d.text("Print formatted version:\n ");
  //   d.lit("ff");d.text("  to print in compact debug format\n ");
  //   d.lit("fff");d.text(" to print in expanded debug format");
  //   d}).switch().many().guard(|x| x.len() <= 3, "> 3 formatting flag repetitions")
  //   .map(|x| if x[0] {x.len()}else{x.len()-1});

  let p_ci  	= s('i').l("pci"   ).h({let mut d=Doc::default();d.text("Print masks for CursorInfo API");
    d})     	. switch();
  let p_dx  	= s('p').l("pdx"   ).h({let mut d=Doc::default();d.text("Print masks for DX Dupplication (screnshot) API");
    d})     	. switch();
  let coord 	= s('c').l("screen").h({let mut d=Doc::default();d.text("Report values in screen coordinates");
    d})     	. switch();
  let shadow	= s('s').l("shadow").h({let mut d=Doc::default();d.text("Add an ≈approximation of shadow size (if it's enabled)");
    d})     	. switch();

  // construct!(Opt {fmt, log, rows}).to_options()
  construct!(Opt {p_ci,p_dx,coord,shadow,rows,}).to_options()
    .version(env!("CARGO_PKG_VERSION"))
    .descr("Quick & dirty debug of the mouse cursor size library using either cursor info API or DX Duplication (screenshot), printing binary/color masks and optionally raw row values")
    // .header("")
    // .footer(&*format!("()"))
    .with_usage(|doc| {let mut u = Doc::default();/*u.emphasis("Use");u.text(": ");*/
      u.lit(env!("CARGO_BIN_NAME"));u.text(" ");u.doc(&doc);
      u
    })
}
