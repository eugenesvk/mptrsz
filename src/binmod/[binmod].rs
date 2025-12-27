extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;

use mptrsz_lib::libmod::get_cursor_reg;
use mptrsz_lib::{φ, libmod::{parse_cursor_h,parse_cursor_dxgi,cur_box_to_screen_hs,is_cursor_shadow,}};

use std::mem;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::{CURSORINFO, HCURSOR,CURSOR_SHOWING,CURSOR_SUPPRESSED,};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetCursorInfo};

pub mod cli_opt;
use cli_opt::*;

use std::{result, error::Error};
pub type Result<T> = result::Result<T, Box<dyn Error>>;
pub fn print42() -> Result<()> {p!("{}",42)?; Ok(())}

pub fn main_cli() -> Result<()> {
  let opt = options().run();
  // p!("parsed args: rows={:?}",opt.rows)?;

  // TODO: when cursor is invisible, use alternative method of measuring its size
  // use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics,};
  // use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN,SM_CYSCREEN,SM_CXICON,SM_CYICON,};
  // let screen_w	= unsafe{GetSystemMetrics(SM_CXSCREEN)};
  // let screen_h	= unsafe{GetSystemMetrics(SM_CYSCREEN)};
  // let cursor_w	= unsafe{GetSystemMetrics(SM_CXICON)};
  // let cursor_h	= unsafe{GetSystemMetrics(SM_CYICON)};
  // pp!("metrics screen w{screen_w} h{screen_h} cursor w{cursor_w} h{cursor_h}");
  let shadow = if is_cursor_shadow(false) {"❏"}else{"□"};
  let dpi = 0; // TODO: add screen scaling
  let acc = match get_cursor_reg() {
    Ok(acc)	=> acc,
    Err(e) 	=> 1,
  };
  p!("{shadow}   ⋅{dpi} dpi   ⋅{acc} sz accessibility")?;

  // 0 Current cursor position (GetCursorPos)
  let mut cur_pos = POINT::default();
  let cur_pos_res =  unsafe{GetCursorPos(&mut cur_pos)}; //current of global?
  if cur_pos_res.is_ok() {pp!("0) 🖰 x{} y{} (GetCursorPos)",cur_pos.x,cur_pos.y);}

  // 1 🖰 Global cursor (GetCursorInfo) even if it's not owned by the current thread
  // 1.1 Get handle to the cursor itself
  let mut curℹ = CURSORINFO {cbSize: mem::size_of::<CURSORINFO>() as u32, ..Default::default()};
    /*hCursor:HCURSOR   cbSize:u32 (!must set before! ??? becomes 0 after GetCursorInfo call)
    flags      :CURSORINFO_FLAGS	0=hidden 1=CURSOR_SHOWING 2=CURSOR_SUPPRESSED (touch/pen)
    ptScreenPos:POINT           	screen coordinates of the cursor*/
  let res = unsafe { GetCursorInfo(&mut curℹ) }; if res.is_err() {pp!("1.1) ✗ GetCursorInfo");}else{
    let cur_h:HCURSOR = curℹ.hCursor;
    let vis = if curℹ.flags.0 == 0                	{"✗🕶" //hidden
      } else  if curℹ.flags   == CURSOR_SHOWING   	{"✓👓"
      } else  if curℹ.flags   == CURSOR_SUPPRESSED	{"✗supr"
      } else                                      	{""};
    let x = curℹ.ptScreenPos.x; let y = curℹ.ptScreenPos.y;
    φ!("1.1) 🖰 global: x{x} y{y} {vis} +handle (GetCursorInfo)");
    if curℹ.flags != CURSOR_SHOWING {return Ok(())}

    // 1.2 Get handle(s) to the cursor bitmap mask(s)
    let coords = parse_cursor_h(cur_h, opt.p_ci, &opt.rows, opt.shadow);
    match coords {
      Ok(mut c)	=> {if opt.coord {cur_box_to_screen_hs(&mut c, &curℹ.ptScreenPos)};
        /**/   	    pp!("global 🖰 𝑏map CI: {:?}",c);},
      Err(e)   	=> {pp!("global 🖰 𝑏map CI: no mouse pointer shape captured: {e}");},
    };
  }

  // 2 🖰 Current cursor (mostly busy even if it's invisible during fast run)
  // let cur_h:HCURSOR =  unsafe{GetCursor()}; if cur_h.is_invalid() {pp!("2.1) ✗ GetCursor");}else{
  //   pp!("2.1) 🖰 current: +handle (GetCursor)");
  //   // 2.2 Get handle(s) to the cursor bitmap mask(s)
  //   let coords = parse_cursor_h(cur_h);
  //   match coords {
  //     Some(c)	=> {pp!("current 🖰 𝑏map: coords {:?}",c);},
  //     None   	=> {pp!("current 🖰 𝑏map: no mouse pointer shape captured");},
  //   };
  // }


  // 3 DXGI duplication API (screenshot the whole screen, get pointer image). Unlike ↑ captures shadow
    let coords = parse_cursor_dxgi(opt.p_dx, opt.coord, &opt.rows);
    match coords {
      Ok (c)	=> {pp!("global 🖰 𝑏map DX: {c:?}");},
      Err(𝑒)	=> {pp!("global 🖰 𝑏map DX: no mouse pointer shape captured: {𝑒}");},
    };

  Ok(())
}
