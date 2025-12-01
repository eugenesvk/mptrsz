#![cfg_attr(not(debug_assertions),allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

_mod!(binmod); //→ #[path="binmod/[binmod].rs"] pub mod binmod;
use crate::binmod::print42;
use dummy_lib::libmod::{ret42,get_mptr_sz,measure_mcursor_bm,};

use std::error::Error;
use std::result;
use std::mem;
use std::mem::{size_of, zeroed};

// type Result<T> = result::Result<T, Box<dyn Error>>;
// fn main() -> Result<()> {
//   print42()?;
//   get_mptr_sz();
//   ret42();
//   Ok(())
// }

// TODO:
  // !!! remove screen capture, only capture the pointer
    // !!: use another non desktop-duplication API, which is an overkill for this purpose since we don't need to capture the whole screen on the GPU
    // detect which monitor has pointer?

use windows::Win32::Foundation::{POINT, BOOL, TRUE, FALSE,};
use windows::Win32::Graphics::Gdi::{DeleteObject,GetObjectW,BITMAP,HGDIOBJ,HBITMAP,};
use windows::Win32::UI::WindowsAndMessaging::{HICON, ICONINFO, CURSORINFO, HCURSOR, CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,};
use windows::Win32::UI::WindowsAndMessaging::{GetCursor, GetCursorPos, GetCursorInfo, GetIconInfo};

use std::path::PathBuf;
use docpos::*;
use dummy_lib::libmod::CursorColor;


fn main() {
  //  Current cursor position (GetCursorPos)
  let mut mptr_pos = POINT::default();
  let mptr_pos_res =  unsafe{GetCursorPos(&mut mptr_pos)}; //current of global?
  if mptr_pos_res.is_ok() {println!("x{} y{} (GetCursorPos)",mptr_pos.x,mptr_pos.y);}

  // 1 Global cursor (GetCursorInfo)
  //?why fails? console? 2 Global cursor, even if it is not owned by the current thread
  let mut mcurℹ = CURSORINFO::default();
  let mptr_inf_res=  unsafe{GetCursorInfo(&mut mcurℹ)}; // CURSORINFO {cbSize:u32
    // flags      	: CURSORINFO_FLAGS	hidden 1 CURSOR_SHOWING 2 CURSOR_SUPPRESSED (touch/pen)
    // hCursor    	: HCURSOR         	.
    // ptScreenPos	: POINT           	screen coordinates of the cursor
  if mptr_inf_res.is_ok() {
    let vis = if mcurℹ.flags.0 == 0                	{"✗🕶"
      } else  if mcurℹ.flags   == CURSOR_SHOWING   	{"✓👓"
      } else  if mcurℹ.flags   == CURSOR_SUPPRESSED	{"✗supr"
      } else                                       	{""};
    let x = mcurℹ.ptScreenPos.x;
    let y = mcurℹ.ptScreenPos.y;
    println!("global 🖰: x{x} y{y} is{vis} (GetCursorInfo)");
  } else {println!("✗ GetCursorInfo"); }


  // 2 Current cursor
  let hCursor:HCURSOR =  unsafe{GetCursor()}; //Current cursor
  let mut iℹ = ICONINFO::default();
  let iℹ_result = unsafe{GetIconInfo(hCursor.into(), &mut iℹ)};
    // GetIconInfo(hicon: HICON, piconinfo: *mut ICONINFO) -> Result<()>
    // fIcon   	: BOOL   	,//TRUE specifies an icon; FALSE specifies a cursor.
    // xHotspot	: u32    	, yHotspot:u32
    // hbmMask 	: hBITMAP	,// icon monochrome mask bitmap. Monochrome icons: hbmMask = 2⋅iconHeight = AND mask on top and XOR mask on the bottom
    // hbmColor	: hBITMAP	,// icon color           bitmap. NULL for monochrome
  if iℹ_result.is_ok() {
    let ic_tp = if iℹ.fIcon == TRUE {"🖼"}else{"🖰 pointer"};
    let hot_x = iℹ.xHotspot;
    let hot_y = iℹ.yHotspot;
    let is_mask = !iℹ.hbmMask.is_invalid();
    let is_col  = !iℹ.hbmColor.is_invalid();
    let is_monoc = iℹ.hbmColor.is_invalid();
    println!("{ic_tp} hot x{hot_x} y{hot_y} mono={is_monoc} (GetIconInfo of GetCursor current)",);

    // TODO: check if correct
    let cur_col = if  is_mask && !is_col	{CursorColor::Mono
      } else      if !is_mask &&  is_col	{CursorColor::Color
      } else      if  is_mask &&  is_col	{CursorColor::ColorMasked
      } else                            	{CursorColor::Color};
    let mut out_str = String::new();
    let dbg = true;
    let bm_h = if let CursorColor::Mono = cur_col {iℹ.hbmMask} else {iℹ.hbmColor};
    let coords = if dbg	{measure_mcursor_bm(bm_h, cur_col, Some(&mut out_str))
    } else             	{measure_mcursor_bm(bm_h, cur_col, None)};
    println!("{}",out_str);
    match coords {
      Some(c)	=> {println!("coords {:?}",c);},
      None   	=> {println!("no mouse pointer shape captured");},
    };



    // Avoid resource leaks    DeleteObject(ho:HGDIOBJ) -> BOOL
    let _d1 = if iℹ.hbmMask .is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmMask .into())}};
    let _d2 = if iℹ.hbmColor.is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmColor.into())}};
    if _d1==FALSE || _d2==FALSE {println!("🛑GDI resource leak! ✗Mask {_d1:?} ✗Color {_d2:?}");}
  }

  // let mut out_str = String::new();
  // let dbg = true;
  // let coords = if dbg	{get_mptr_sz(Some(&mut out_str))
  // } else             	{get_mptr_sz(None)};
  // println!("{}",out_str);
  // match coords {
  //   Some(c)	=> {println!("coords {:?}",c);},
  //   None   	=> {println!("no mouse pointer shape captured");},
  // };
}
