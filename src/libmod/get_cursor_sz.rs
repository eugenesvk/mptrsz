extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
 // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

use crate::libmod::{get_mptr_sz,measure_mcursor_bm,cur_box,Point,CursorSizeErr,};
use crate::φ;


const dbg:bool = true;

use windows::Win32::Foundation::{TRUE,FALSE,};
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::UI::WindowsAndMessaging::{ICONINFO, HCURSOR,};
use windows::Win32::UI::WindowsAndMessaging::GetIconInfo;


pub fn parse_cursor_h(cur_h:HCURSOR, p:bool, rows:&[usize]) -> Result<cur_box, CursorSizeErr> {
  let mut iℹ = ICONINFO::default();
    /*fIcon :BOOL   	TRUE specifies an icon; FALSE specifies a cursor
    xHotspot:u32    	, yHotspot:u32
    hbmMask :hBitMap	icon monochrome mask bitmap. Monochrome icons: hbmMask = 2⋅iconHeight = AND mask on top and XOR mask on the bottom
    hbmColor:hBitMap	icon color           bitmap. NULL for monochrome*/
  let res = unsafe { GetIconInfo(cur_h.into(), &mut iℹ) }; if res.is_err() {if p{pp!("1) ✗ GetIconInfo")}; Err(CursorSizeErr::Ii("✗ Windows GetIconInfo call failed".into()))}else{
    if p {let iℹ_T	= if iℹ.fIcon == TRUE {'🖼'}else{'🖰'};
      let hot_x   	=    iℹ.xHotspot; let hot_y = iℹ.yHotspot;
      φ!("2) T={iℹ_T} {}  hot_x{hot_x} y{hot_y} (GetIconInfo)",if iℹ_T=='🖰'{"≝🖰"}else{"!!! should be 🖰 !!!"});}
    let hot_p = Point {x:iℹ.xHotspot as i32, y:iℹ.yHotspot as i32};

    // 3 Get handle(s) to the cursor bitmap mask(s)
    let coords = if dbg && p {let mut out_str = String::new();
      let _r	=measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, hot_p, Some(&mut out_str),rows); pp!("{}",out_str); _r
    } else  	{measure_mcursor_bm(iℹ.hbmMask, iℹ.hbmColor, hot_p, None              ,rows)};
    // let bm_h = if iℹ.hbmColor.is_invalid() {iℹ.hbmMask} else {iℹ.hbmColor};
    // test_GetDIBits(bm_h);

    // Avoid resource leaks    DeleteObject(ho:HGDIOBJ) -> BOOL
    let _d1 = if iℹ.hbmMask .is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmMask .into())}};
    let _d2 = if iℹ.hbmColor.is_invalid(){TRUE}else{unsafe{DeleteObject(iℹ.hbmColor.into())}};
    // todo: convert to proper error or leave as is, do we need to abort on leaks?
    if (_d1==FALSE || _d2==FALSE) && p {pp!("🛑GDI resource leak! ✗Mask {_d1:?} ✗Color {_d2:?}");}

    coords
  }
}

pub fn parse_cursor_dxgi(p:bool, coord:bool, rows:&[usize]) -> Result<cur_box, CursorSizeErr> {
  if dbg && p {pp!("\n\n\n——————————————— 2. DXGI duplication API (screenshot)\n");}
  if dbg && p {let mut out_str = String::new();
    let _r	=get_mptr_sz(Some(&mut out_str),coord,rows); pp!("{}",out_str); _r
  } else  	{get_mptr_sz(None              ,coord,rows)}
}
