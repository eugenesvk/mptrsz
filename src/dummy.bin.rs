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
use dummy_lib::libmod::{ret42,get_mptr_sz,measure_mcursor_bm,cur_box,};

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
use windows::Win32::Graphics::Gdi::{DeleteObject,GetObjectW,GetBitmapBits,GetDIBits};
use windows::Win32::UI::WindowsAndMessaging::{HICON, ICONINFO, CURSORINFO, HCURSOR, CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,};
use windows::Win32::UI::WindowsAndMessaging::{GetCursor, GetCursorPos, GetCursorInfo, GetIconInfo};

use std::path::PathBuf;
use docpos::*;
use dummy_lib::libmod::CursorColor;

fn parse_cursor_h(cur_h:HCURSOR) -> Option<cur_box> {
  let mut iℹ = ICONINFO::default();
    /*fIcon :BOOL   	TRUE specifies an icon; FALSE specifies a cursor
    xHotspot:u32    	, yHotspot:u32
    hbmMask :hBitMap	icon monochrome mask bitmap. Monochrome icons: hbmMask = 2⋅iconHeight = AND mask on top and XOR mask on the bottom
    hbmColor:hBitMap	icon color           bitmap. NULL for monochrome*/
  // todo: convert to a proper error
  let res = unsafe { GetIconInfo(cur_h.into(), &mut iℹ) }; if !res.is_ok() {println!("1) ✗ GetIconInfo");None}else{
    let iℹ_T   	= if iℹ.fIcon == TRUE {'🖼'}else{'🖰'};
    let hot_x  	=    iℹ.xHotspot; let hot_y = iℹ.yHotspot;
    let is_mono	=    iℹ.hbmColor.is_invalid();
    let is_col 	=   !iℹ.hbmColor.is_invalid();
    let is_mask	=   !iℹ.hbmMask .is_invalid();
    // TODO: this is definitely wrong, ColorMasked is defined by the α-channel state at the source, if it's used for transparency, then it's a 32b Color cursor, if it's used for a 0/1 mask, then it's a ColorMasked type, but in both of these cases the cursor bitmap will be 32b BGRα
    let cur_col = if  is_mask && !is_col	{CursorColor::Mono
      } else      if !is_mask &&  is_col	{CursorColor::Color
      } else      if  is_mask &&  is_col	{CursorColor::ColorMasked
      } else                            	{CursorColor::Color};
    println!("2) T={iℹ_T} {}  hot_x{hot_x} y{hot_y} CT={cur_col:?} (GetIconInfo)",if iℹ_T=='🖰'{"≝🖰"}else{"!!! should be 🖰 !!!"});

    // 3 Get handle(s) to the cursor bitmap mask(s)
    let bm_h = if let CursorColor::Mono = cur_col {iℹ.hbmMask} else {iℹ.hbmColor};
    let coords = if dbg {let mut out_str = String::new();
      let _r	=measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, &cur_col, Some(&mut out_str)); println!("{}",out_str); _r
    } else  	{measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, &cur_col, None)};

    // Avoid resource leaks    DeleteObject(ho:HGDIOBJ) -> BOOL
    let _d1 = if iℹ.hbmMask .is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmMask .into())}};
    let _d2 = if iℹ.hbmColor.is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmColor.into())}};
    if _d1==FALSE || _d2==FALSE {println!("🛑GDI resource leak! ✗Mask {_d1:?} ✗Color {_d2:?}");}

    coords
  }
}

fn parse_cursor_dxgi() -> Option<cur_box> {
  if dbg {println!("\n\n\n——————————————— 2. DXGI duplication API\n———————————————\n\n");}
  if dbg {let mut out_str = String::new();
    let _r	=get_mptr_sz(Some(&mut out_str)); println!("{}",out_str); _r
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
  if cur_pos_res.is_ok() {println!("0) 🖰 x{} y{} (GetCursorPos)",cur_pos.x,cur_pos.y);}

  // 1 🖰 Global cursor (GetCursorInfo) even if it's not owned by the current thread
  // 1.1 Get handle to the cursor itself
  let mut curℹ = CURSORINFO::default(); curℹ.cbSize = mem::size_of::<CURSORINFO>() as u32;
    /*hCursor:HCURSOR   cbSize:u32 (!must set before! ??? becomes 0 after GetCursorInfo call)
    flags      :CURSORINFO_FLAGS	0=hidden 1=CURSOR_SHOWING 2=CURSOR_SUPPRESSED (touch/pen)
    ptScreenPos:POINT           	screen coordinates of the cursor*/
  let res = unsafe { GetCursorInfo(&mut curℹ) }; if !res.is_ok() {println!("1.1) ✗ GetCursorInfo");}else{
    let cur_h:HCURSOR = curℹ.hCursor;
    let vis = if curℹ.flags.0 == 0                	{"✗🕶" //hidden
      } else  if curℹ.flags   == CURSOR_SHOWING   	{"✓👓"
      } else  if curℹ.flags   == CURSOR_SUPPRESSED	{"✗supr"
      } else                                      	{""};
    let x = curℹ.ptScreenPos.x; let y = curℹ.ptScreenPos.y;
    println!("1.1) 🖰 global: x{x} y{y} {vis} +handle (GetCursorInfo)");

    // 1.2 Get handle(s) to the cursor bitmap mask(s)
    let coords = parse_cursor_h(cur_h);
    match coords {
      Some(c)	=> {println!("global 🖰 𝑏map: coords {:?}",c);},
      None   	=> {println!("global 🖰 𝑏map: no mouse pointer shape captured");},
    };
  }

  // 2 🖰 Current cursor
  let cur_h:HCURSOR =  unsafe{GetCursor()}; if cur_h.is_invalid() {println!("2.1) ✗ GetCursor");}else{
    println!("2.1) 🖰 current: +handle (GetCursor)");
    // 2.2 Get handle(s) to the cursor bitmap mask(s)
    let coords = parse_cursor_h(cur_h);
    match coords {
      Some(c)	=> {println!("current 🖰 𝑏map: coords {:?}",c);},
      None   	=> {println!("current 🖰 𝑏map: no mouse pointer shape captured");},
    };
  }


  // 3 DXGI duplication API (screenshot the whole screen, get pointer image). Unlike ↑ captures shadow
    let coords = parse_cursor_dxgi();
    match coords {
      Some(c)	=> {println!("DXGI: coords {:?}",c);},
      None   	=> {println!("DXGI: no mouse pointer shape captured");},
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


use core::ffi::c_void;
use std::ptr::null_mut;
use windows::Win32::Graphics::Gdi::{HDC};
use windows::Win32::Graphics::Gdi::{GetDC};



fn test_more_complicated_way_to_get_bitmap_bits(bmp_h:HBITMAP) {
  // currently using deprecated, but much simpler GetBitmapBits API
  // Convert HBITMAP → BGRA bytes
  let mut bmp: BITMAP = unsafe{std::mem::zeroed()};
    // bmType:i32=0   bmPlanes:u16=№color planes (NOT!!! colors)
    // bmWidth ¦ bmHeight	:i32        	//>0 pixels
    // bmWidthBytes      	:i32        	//№bytes in each scan line. ==EVEN because OS assumes that bit values of a bitmap form an array that is word aligned
    // bmBitsPixel       	:u16        	//𝑏⁄𝑝  №𝑏 bits required to indicate the color of a pixel
    // bmBits            	:*mut c_void	//pointer to location of bit values for the bitmap. Its member must be a pointer to an array of character (1-byte) values
  let bmp_sz = unsafe { GetObjectW(bmp_h.into(), std::mem::size_of::<BITMAP>() as i32
    , Some(&mut bmp as *mut BITMAP as *mut c_void));};

  let width  = bmp.bmWidth      as usize;
  let height = bmp.bmHeight     as usize;
  let stride = bmp.bmWidthBytes as usize;
  let bpp    = bmp.bmBitsPixel;
  let buf_size = stride * height;
  let ptr_bmbits = bmp.bmBits; // !! null since we didn't use CreateDIBSection to get bmp_h

  // Get actual bits
  // 1. Deprecated API, but much simpler without DC surfaces
  let mut cursor_pixels = vec![0u8; buf_size];
  let bytes = unsafe{ GetBitmapBits(bmp_h, cursor_pixels.len() as i32,  cursor_pixels.as_mut_ptr() as *mut c_void,) };
  // unsafe{std::ptr::copy_nonoverlapping(bmp.bmBits as *const u8, cursor_pixels.as_mut_ptr(), buf_size);}

  // 2. GetDIBits
  // let dc_window: HDC = GetDC(null_mut());

  // let bitmap_size: usize = (((bitmap.bmWidth * 32 + 31) / 32) * 4 * bitmap.bmHeight) as usize;
  // println!("bitmap size: {}", bitmap_size);
  // let mut buffer: Vec<u8> = vec![0; bitmap_size];

  // let h_dib = GlobalAlloc(GHND, bitmap_size);
  // let lpbitmap = GlobalLock(h_dib);
  // println!("bitmap {:p}", lpbitmap);
  // let mut buffer: Vec<u8> = vec![0; bitmap_size];

  // GetDIBits(dc_window, hbm,
  //   0,
  //   bitmap.bmHeight as u32,
  //   // lpbitmap,
  //   buffer.as_mut_ptr() as *mut c_void,
  //   (&mut bi) as *mut BITMAPINFOHEADER as *mut BITMAPINFO,
  //   DIB_RGB_COLORS,
  // );
  /*       	int         	i32                	GetDIBits
    hdc    	HDC         	HDC                	handle to the device context
    hbm    	HBITMAP     	HBITMAP            	handle to the bitmap; must be a compatible bitmap (DDB)
    start  	UINT        	u32                	1st  scan line  to retrieve
    cLines 	UINT        	u32                	№ of scan lines to retrieve
   ←lpvBits	LPVOID      	Option<*mut c_void>	pointer to a buffer to receive the bitmap data. If NULL, pass  dimensions/format of the bitmap to the BITMAPINFO structure pointed to by the lpbmi parameter
   ↔lpbmi  	LPBITMAPINFO	*mut BITMAPINFO    	pointer to a BITMAPINFO struct that specifies the desired format for the DIB data
    usage  	UINT        	DIB_USAGE          	format of the bmiColors member of the BITMAPINFO structure (PAL/RGB)
  */

  // let mut buffer = vec![0u8; buf_size];
  // unsafe{std::ptr::copy_nonoverlapping(bmp.bmBits as *const u8, buffer.as_mut_ptr(), buf_size);}

  let _ = unsafe{DeleteObject(bmp_h.into())};
}
