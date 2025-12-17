#![cfg_attr(not(debug_assertions),allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types,mixed_script_confusables,confusable_idents))]
#![cfg_attr(    debug_assertions ,allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types,mixed_script_confusables,confusable_idents,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::print42;
use mouse_sz_lib::libmod::{ret42,get_mptr_sz,measure_mcursor_bm,cur_box,Point,parse_cursor_h,parse_cursor_dxgi};
use mouse_sz_lib::φ;

use thiserror::Error;
use std::result;
use std::mem;
use std::mem::{size_of, zeroed};

const dbg:bool = true;
// type Result<T> = result::Result<T, Box<dyn Error>>;
// fn main() -> Result<()> {
//   print42()?;
//   get_mptr_sz();
//   ret42();
//   Ok(())
// }

use windows::Win32::Foundation::{POINT,BOOL,TRUE,FALSE,};
use windows::Win32::Graphics::Gdi::{BITMAP,HGDIOBJ,HBITMAP,};
use windows::Win32::Graphics::Gdi::{DeleteObject,GetObjectW,GetBitmapBits,GetDIBits,ReleaseDC,};
use windows::Win32::UI::WindowsAndMessaging::{HICON, ICONINFO, CURSORINFO, HCURSOR, CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,};
use windows::Win32::UI::WindowsAndMessaging::{GetCursor, GetCursorPos, GetCursorInfo, GetIconInfo};

use std::path::PathBuf;
use docpos::*;

pub fn main() {
  // TODO: when cursor is invisible, use alternative method of measuring its size
    // system metrics? add enum in return type to know: ≝, bitmap parsing, 3rd???
    // if ( !size.x ) { // use default icon size on this hardware
      // const wxWindow* win = wxApp::GetMainTopWindow();
      // size.x = wxGetSystemMetrics(SM_CXICON, win);
      // size.y = wxGetSystemMetrics(SM_CYICON, win);
    // }

  // 0 Current cursor position (GetCursorPos)
  let mut cur_pos = POINT::default();
  let cur_pos_res =  unsafe{GetCursorPos(&mut cur_pos)}; //current of global?
  if cur_pos_res.is_ok() {pp!("0) 🖰 x{} y{} (GetCursorPos)",cur_pos.x,cur_pos.y);}

  // 1 🖰 Global cursor (GetCursorInfo) even if it's not owned by the current thread
  // 1.1 Get handle to the cursor itself
  let mut curℹ = CURSORINFO::default(); curℹ.cbSize = mem::size_of::<CURSORINFO>() as u32;
    /*hCursor:HCURSOR   cbSize:u32 (!must set before! ??? becomes 0 after GetCursorInfo call)
    flags      :CURSORINFO_FLAGS	0=hidden 1=CURSOR_SHOWING 2=CURSOR_SUPPRESSED (touch/pen)
    ptScreenPos:POINT           	screen coordinates of the cursor*/
  let res = unsafe { GetCursorInfo(&mut curℹ) }; if !res.is_ok() {pp!("1.1) ✗ GetCursorInfo");}else{
    let cur_h:HCURSOR = curℹ.hCursor;
    let vis = if curℹ.flags.0 == 0                	{"✗🕶" //hidden
      } else  if curℹ.flags   == CURSOR_SHOWING   	{"✓👓"
      } else  if curℹ.flags   == CURSOR_SUPPRESSED	{"✗supr"
      } else                                      	{""};
    let x = curℹ.ptScreenPos.x; let y = curℹ.ptScreenPos.y;
    φ!("1.1) 🖰 global: x{x} y{y} {vis} +handle (GetCursorInfo)");
    if curℹ.flags != CURSOR_SHOWING {return}

    // 1.2 Get handle(s) to the cursor bitmap mask(s)
    let coords = parse_cursor_h(cur_h, true);
    match coords {
      Ok(c) 	=> {pp!("global 🖰 𝑏map: coords {:?}",c);},
      Err(e)	=> {pp!("global 🖰 𝑏map: no mouse pointer shape captured: {e}");},
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
    let coords = parse_cursor_dxgi();
    match coords {
      Ok (c)	=> {pp!("DXGI: coords {:?}",c);},
      Err(𝑒)	=> {pp!("DXGI: no mouse pointer shape captured: {}",𝑒);},
    };
  // TODO: parse bitmap from this handle
}
