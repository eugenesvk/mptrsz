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
use mouse_sz_lib::libmod::{ret42,get_mptr_sz,measure_mcursor_bm,cur_box,Point,};
use mouse_sz_lib::φ;

use std::error::Error;
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
use mouse_sz_lib::libmod::CursorColor;

fn parse_cursor_h(cur_h:HCURSOR) -> Option<cur_box> {
  let mut iℹ = ICONINFO::default();
    /*fIcon :BOOL   	TRUE specifies an icon; FALSE specifies a cursor
    xHotspot:u32    	, yHotspot:u32
    hbmMask :hBitMap	icon monochrome mask bitmap. Monochrome icons: hbmMask = 2⋅iconHeight = AND mask on top and XOR mask on the bottom
    hbmColor:hBitMap	icon color           bitmap. NULL for monochrome*/
  // todo: convert to a proper error
  let res = unsafe { GetIconInfo(cur_h.into(), &mut iℹ) }; if !res.is_ok() {pp!("1) ✗ GetIconInfo");None}else{
    let iℹ_T 	= if iℹ.fIcon == TRUE {'🖼'}else{'🖰'};
    let hot_x	=    iℹ.xHotspot; let hot_y = iℹ.yHotspot;
    φ!("2) T={iℹ_T} {}  hot_x{hot_x} y{hot_y} (GetIconInfo)",if iℹ_T=='🖰'{"≝🖰"}else{"!!! should be 🖰 !!!"});
    let mut hot_p = Point {x:iℹ.xHotspot as i32, y:iℹ.yHotspot as i32};

    // 3 Get handle(s) to the cursor bitmap mask(s)
    let coords = if dbg {let mut out_str = String::new();
      let _r	=measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, hot_p, Some(&mut out_str)); pp!("{}",out_str); _r
    } else  	{measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, hot_p, None              )};
    // let bm_h = if iℹ.hbmColor.is_invalid() {iℹ.hbmMask} else {iℹ.hbmColor};
    // test_GetDIBits(bm_h);

    // Avoid resource leaks    DeleteObject(ho:HGDIOBJ) -> BOOL
    let _d1 = if iℹ.hbmMask .is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmMask .into())}};
    let _d2 = if iℹ.hbmColor.is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmColor.into())}};
    if _d1==FALSE || _d2==FALSE {pp!("🛑GDI resource leak! ✗Mask {_d1:?} ✗Color {_d2:?}");}

    coords
  }
}

fn parse_cursor_dxgi() -> Option<cur_box> {
  if dbg {pp!("\n\n\n——————————————— 2. DXGI duplication API\n");}
  if dbg {let mut out_str = String::new();
    let _r	=get_mptr_sz(Some(&mut out_str)); pp!("{}",out_str); _r
  } else  	{get_mptr_sz(None)}
}


fn main() {
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
    let coords = parse_cursor_h(cur_h);
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
  //
  // TODO: HOW to detect whether a cursor is 24bit color (all α=0 even though it exists) or 32 bit color (α
      // no, an empty cursor has all α=0
    // is it safe to assume if no α channel exists, then it must be 32bit color?
      // 𝑎 always exists as part of the bitmap, the question is which values it supports
  //
  // TODO: parse bitmap from this handle
  // todo: how to get mask size with shadow like DXGI does?
    // dxgi: how to ignore shadow and get only the size of the cursor itself?

  // dxdiag outputs actual pointer size, whie geticoninfo only gets I-beam bitmap size?

}


use core::ffi::{c_void,c_int,};
use std::ptr::null_mut;
use windows::Win32::Graphics::Gdi::{HDC};
use windows::Win32::Graphics::Gdi::{GetDC};



