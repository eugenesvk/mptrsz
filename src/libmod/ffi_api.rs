/*! Get the true bounding box size that fits a Windows 🖰mouse cursor=pointer using 2 methods:
  1. GetCursorInfo → GetIconInfo APIs that extracts nominal cursor size that is adjusted by screen DPI, but not adjusted for user accessibility multiplier, and them manually does the approximate adjustment for it. But does NOT take into account cursor shadow.
  2. DirectX duplication API that captures a screenshot and extracts mouse cursor from it. Takes into account all size-related factors: monitor scaling, user accessibility multiplier, cursor shadow.

  See win_api_const.ahk for an example on how to use in AutoHotkey
*/

use crate::libmod::ffi_api::std::mem;
use crate::libmod::*;
use helperes::alias::io;
use helperes::p;
use helperes::alias::type_name;


use widestring::{U16Str,WideChar,u16cstr,
  U16CString,U16CStr,	//   0 U16/U32-CString wide version of the standard CString type
  Utf16Str   ,       	// no0 UTF-16 encoded, growable owned string
};


use std     	::{self,slice,ptr,cmp};
use std::ffi	::{CString};


fn ret_error(err_msg:&U16CStr, err_sz:u32,err_ptr:*mut WideChar) -> *const WideChar { // create a buffer from pointer/size and fill it in
  let err_msg_bufer   	= unsafe{slice::from_raw_parts_mut::<WideChar>(err_ptr, err_sz as usize)};
  let err_msg_b:&[u16]	= err_msg.as_slice_with_nul(); // → slice of underlying elements, incl ␀ terminator
  let max_buff_len    	= cmp::min(err_msg_b.len(),(err_sz / 2) as usize);
  err_msg_bufer[..max_buff_len].copy_from_slice(&err_msg_b[..max_buff_len]);
  err_msg_bufer[max_buff_len-1] = U16CStr::NUL_TERMINATOR;
  ptr::null()
}


use windows::Win32::UI::WindowsAndMessaging::{HICON,ICONINFO,CURSORINFO,HCURSOR,CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,};
use windows::Win32::UI::WindowsAndMessaging::{GetCursor,GetCursorPos,GetCursorInfo,GetIconInfo};
use windows::Win32::Foundation::{POINT,BOOL,TRUE,FALSE,};


pub fn cur_box_to_screen(cbox:&mut cur_box, hs_screen: &POINT) {
  let icon_box_x = hs_screen.x - cbox.hs.x;
  let icon_box_y = hs_screen.y - cbox.hs.y;
  cbox.ptl.x += icon_box_x;
  cbox.ptl.y += icon_box_y;
  cbox.pbr.x += icon_box_x;
  cbox.pbr.y += icon_box_y;
  cbox.hs.x  = hs_screen.x;
  cbox.hs.y  = hs_screen.y;
}

use std::path::PathBuf;
use docpos::*;

#[repr(u8)] #[derive(Copy,Clone,Debug)]
pub enum err {Ok = 0, Er = 1 }

#[repr(C)] #[derive(Copy,Clone,Debug)] #[docpos]
pub struct maybe_cur_box { /// cur_box with a tag denoting whether it's a valid copy or an 0-ed error struct. Default is with an error.
  pub err:err ,///  tag signaling whether the cursor size box is valid
                     ///! Cursor size box (0-ed if tag is invalid)
  pub cur_box:cur_box ,
}
impl Default for maybe_cur_box {fn default() -> Self {
  maybe_cur_box {err: err::Er, cur_box: cur_box::default()}}
}

#[unsafe(no_mangle)] pub extern "C"
fn get_mcursor_sz_ci(coord:i8, err_sz:u32,err_ptr:*mut WideChar) -> maybe_cur_box {
  // 1 🖰 Global cursor (GetCursorInfo) even if it's not owned by the current thread
  // 1.1 Get handle to the cursor itself
  let mut curℹ = CURSORINFO {cbSize: mem::size_of::<CURSORINFO>() as u32, ..Default::default()};
    /*hCursor:HCURSOR   cbSize:u32 (!must set before! ??? becomes 0 after GetCursorInfo call)
    flags      :CURSORINFO_FLAGS	0=hidden 1=CURSOR_SHOWING 2=CURSOR_SUPPRESSED (touch/pen)
    ptScreenPos:POINT           	screen coordinates of the cursor*/
  let res = unsafe { GetCursorInfo(&mut curℹ) }; if res.is_err()                 {let _ = ret_error(u16cstr!("✗ Couldn't ‘GetCursorInfo’!"                 ),err_sz,err_ptr); return maybe_cur_box::default()}
  let cur_h:HCURSOR = curℹ.hCursor;              if curℹ.flags != CURSOR_SHOWING {let _ = ret_error(u16cstr!("✗ cursor is not shown (hidden or touch/pen)!"),err_sz,err_ptr); return maybe_cur_box::default()}

  // 1.2 Get/parse handle(s) to the cursor bitmap mask(s)
  let coords = parse_cursor_h(cur_h, false);
  match coords {
    Ok(mut c)	=> {if coord == 0 {cur_box_to_screen(&mut c, &curℹ.ptScreenPos)}; maybe_cur_box{err:err::Ok, cur_box:c}},
    Err(e)   	=> {let _ = ret_error(u16cstr!("✗ Couldn't get 🖰 cursor size box parsing bitmaps from ‘GetCursorInfo’ → ‘GetIconInfo’!"),err_sz,err_ptr);  //todo: provide reasons by adding errors to get_mptr_sz
      maybe_cur_box::default()},
  }
}

#[unsafe(no_mangle)] pub extern "C"
fn get_mcursor_sz_dx(coord:i8, err_sz:u32,err_ptr:*mut WideChar) -> maybe_cur_box {
  // 2 DXGI duplication API (screenshot the whole screen, get pointer image). Unlike ↑ captures shadow
  match get_mptr_sz(None) {
    Ok(mut c) => {
      if coord == 0 { //convert to screen coordinates once we get hotspot's screen coords
        let mut cur_pos = POINT::default();
        let cur_pos_res = unsafe{GetCursorPos(&mut cur_pos)};
        if  cur_pos_res.is_ok() {cur_box_to_screen(&mut c, &cur_pos);
        } else {let _ = ret_error(u16cstr!("✗ Couldn't ‘GetCursorPos’!"),err_sz,err_ptr);
          return maybe_cur_box::default()  }
      };
      maybe_cur_box{err:err::Ok, cur_box:c}},
    Err(𝑒)   => {let _ = ret_error(u16cstr!("✗ Couldn't get 🖰 cursor size box using DX duplication API for an unknown reason!"),err_sz,err_ptr);
      maybe_cur_box::default()  },
      // todo: send error messages as well? or match and send raw string
  }
}

/** # SAFETY
  Must be called only with a pointer generated by another Rust function via `.into_raw`. The pointer can't be used after this call, and the FFI receiver of this pointer can't edit it*/
#[unsafe(no_mangle)] pub unsafe extern "system"
fn dealloc_lib_str(str_ptr:*mut i8) {unsafe{let _ = CString::from_raw(str_ptr);}}
