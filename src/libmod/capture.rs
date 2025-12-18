use crate::libmod::*;
use helperes::alias::io;
use helperes::p;
use helperes::alias::type_name;
use rusty_duplication::{FrameInfoExt, Scanner, VecCapturer, Monitor};
use std::{fs::File, io::Write, thread, time::Duration};
use bitvec::prelude::*; // to iterate over individual pixels packed in a byte
//use bitvec::prelude as 𝑏; // to iterate over individual pixels packed in a byte

/// Color represented by additive channels: Blue (b), Green (g), Red (r), and Alpha (a)
  // DXGI provides a surface that contains a current desktop image through the new IDXGIOutputDuplication::AcquireNextFrame method. The format of the desktop image is always DXGI_FORMAT_B8G8R8A8_UNORM no matter what the current display mode is
  // learn.microsoft.com/en-us/windows/win32/direct3ddxgi/desktop-dup-api
  // DXGI_FORMAT_B8G8R8A8_UNORM Value:87  A four-component, 32-bit unsigned-normalized-integer format that supports 8 bits for each color channel and 8-bit alpha
#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)]
pub struct BGRA8 {pub b:u8,  pub g:u8,  pub r:u8,  pub a:u8,}

use std::mem;

use windows::{
  Win32::Graphics::{
    Dxgi::{
      Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC, DXGI_MODE_ROTATION_ROTATE90,DXGI_MODE_ROTATION_ROTATE270,},
      DXGI_OUTDUPL_POINTER_SHAPE_TYPE,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME,
    },
  },
};

use windows::Win32::Foundation::{POINT,BOOL,TRUE,FALSE,};
use windows::Win32::Graphics::Gdi::{BITMAP,HGDIOBJ,HBITMAP,
  DeleteObject,GetObjectW,GetBitmapBits,GetDIBits};
use windows::Win32::UI::WindowsAndMessaging::{HICON,ICONINFO,CURSORINFO,HCURSOR,CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,
  GetCursor,GetCursorPos,GetCursorInfo,GetIconInfo};
use std::slice;
use core::ffi::c_void;
use std::mem::{size_of,zeroed};


use std::path::PathBuf;
use docpos::*;
#[docpos]
pub fn get_mptr_sz( /// Get the true bounding box of a 🖰 pointer (if visible), i.e., the minimal box that contains all the pointer pixels. If `E̲nable pointer shadow` Windows Mouse setting is on, the cursor size increases by ~9⋅7 pixels, so instead of 48⋅48 (48=32⋅1.5 screen scaling) you'd get 57⋅55 (also affects the cursor positioning within the cursor frame). `GetCursorInfo` alternative seems to ignore shadows and always gets 48⋅48. However, `Colorμ` cursors (24𝑏=8𝑏⋅3𝑐 `TrueColor` colors with at least 1 pixel "inverted" that requires using α-channel to track inversion (0xFF inverts, 0x0 replaces; 𝑎-channel is 0-ed out in regular 24𝑏 color bitmap)) do not drop shadow, so retain the same size (48⋅48 in the example above)
  ///! store the text drawing of the pointer and print a few metrics (mostly for debugging)
  mut s:Option<&mut String>
) -> Result<cur_box,CursorSizeErr>  {
  let is_s = s.is_some(); //store a printout string of non-empty pixels

  let mut mon_scanner         	= Scanner::new()    .unwrap(); // Scanner to scan for monitors
  let     monitor :Monitor    	= mon_scanner.next().unwrap(); // Scanner has Iterator, so iterate through monitors
  let mut capturer:VecCapturer	= monitor.try_into().unwrap(); // Create a vec capturer for a monitor this will allocate memory buffer to store pixel data
  // let output_desc  = capturer.monitor().dxgi_output_desc().unwrap(); // you can also get monitor info from a capturer

  // thread::sleep(Duration::from_millis(50)); // sleep before capture to wait system to update the screen
  let capt = capturer.capture_with_pointer_shape().unwrap(); // Res<(DXGI_OUTDUPL_FRAME_INFO,Option<DXGI_OUTDUPL_POINTER_SHAPE_INFO>,)>

  let maybe_ptr_shape = capt.1;
  match maybe_ptr_shape {None=>{return Err(CursorSizeErr::DXDupe("Failed to capture pointer shape".into())) },
    Some(ptr_shape)	=> {
      let w = ptr_shape.Width; let w_sz = w as usize;
      let h = ptr_shape.Height;
      let wb= ptr_shape.Pitch; //🡘b width in bytes of the mouse cursor
      let hot_x = ptr_shape.HotSpot.x; //super::super::Foundation::POINT,
      let hot_y = ptr_shape.HotSpot.y;
        // position of the cursor's hot spot relative to its upper-left pixel
        // app doesn't use hot spot when it determines where to draw the cursor shape
      let ps_type = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(ptr_shape.Type as i32);
      let pad = if h <= 9 {1} else if h <= 99 {2} else {3};

      let mut scan_line_test = [0,1,3,4];
      let mut chunk_test:Vec<u8> = vec![];
      // !: empty pointer will have nonsensical →0 < ←w, this is not checked    █■•◧□
      let mut most𐎓	= w as usize; //pushed ← if a valid pixel found
      let mut most𑁱	= 0         ; //pushed → …
      let mut most𖭩	= h as usize;
      let mut most𖭪	= 0;
      let mut 𝑐ℕ   	= 1;

      // not needed to account for rotation?
        // let scan_lines = match output_desc.Rotation {
        //     DXGI_MODE_ROTATION_ROTATE90 |
        //     DXGI_MODE_ROTATION_ROTATE270  => ptr_shape.Width,
        //     _                             => ptr_shape.Height,
        //   }; //  DXGI_MODE_ROTATION_ …  UNSPECIFIED=0  IDENTITY=1  ROTATE90=2  ROTATE180=3  ROTATE270=4
        // println!("{:?} Rotation",output_desc.Rotation);

      // Iterate over mouse pointer buffer to detect blank pixels and true box size
      let ptr_buff = capturer.pointer_shape_buffer;

      if        ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME   { //1c·1𝑏pc=1𝑏pp DIB ⋀AND mask + ⊻XOR mask (⋅2))
        // █black □white
        let hmask = (h/2) as usize; // split between ⋀AND and ⊻XOR masks
        let pad = if hmask <= 9 {1} else if hmask <= 99 {2} else {3};
        𝑐ℕ=1; let 𝑏pc=1; let px_sz = 𝑐ℕ * 𝑏pc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {𝑏pc} 𝑏⁄𝑐 {px_sz} ■sz𝑏 {row_sz_b} row_sz𝑏 {hmask}hmask\n");}
        // scan_line_test = [24];

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(𝑖row, chunk)| {
          if is_s && φL>=3&&scan_line_test.contains(&𝑖row) {chunk_test.extend_from_slice(chunk);}
          if is_s {*s.as_deref_mut().unwrap() += "¦";}
          let chunk𝑏 = BitSlice::<_,Msb0>::from_slice(chunk);
          if 𝑖row < hmask {if 𝑖row==0     && is_s {*s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask 1≝ 0Δ• ———¦\n¦";}
            chunk𝑏.chunks(𝑏pc   ).enumerate().for_each(|(𝑗col, px   )| { // px: &BitSlice<u8>
              if  !px[0]{
                if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱	{most𑁱 = 𝑗col};
                if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪	{most𖭪 = 𝑖row};  }
              if is_s {(*s.as_deref_mut().unwrap()).push(if !px[0]{'•'}else{' '})}
            });
          } else          {if 𝑖row==hmask && is_s {*s.as_deref_mut().unwrap() += "———  ⊻XOR Mono◧ bitmask 0≝ 1Δ• ———¦\n¦";}
            let hrow = 𝑖row - hmask;
            chunk𝑏.chunks(𝑏pc   ).enumerate().for_each(|(𝑗col, px   )| { // px: &BitSlice<u8>
              if   px[0]{
                if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱	{most𑁱 = 𝑗col};
                if hrow < most𖭩	{most𖭩 = hrow}; if hrow > most𖭪	{most𖭪 = hrow};  }
              if is_s {(*s.as_deref_mut().unwrap()).push(if  px[0]{'•'}else{' '})}
            });
          }
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
        });

      } else if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR        { //4c·8𝑏pc=32𝑏pp BGRα DIB
        // █black ■~black □~white ◧other color (visually works best for greys)
        𝑐ℕ=4; let 𝑏pc=8; let px_sz = 𝑐ℕ * 𝑏pc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {𝑏pc} 𝑏⁄𝑐 {px_sz} ■sz𝑏 {row_sz_b} row_sz𝑏\n");}
        // scan_line_test = [24];

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(𝑖row, chunk)| {
          if is_s && φL>=3&&scan_line_test.contains(&𝑖row) {chunk_test.extend_from_slice(chunk);}
          if is_s {*s.as_deref_mut().unwrap() += "¦";}
          chunk.chunks(  px_sz  ).enumerate().for_each(|(𝑗col, px   )| {
            if px != px0 {
              if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱	{most𑁱 = 𝑗col};
              if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪	{most𖭪 = 𝑖row};
            }
            if is_s {(*s.as_deref_mut().unwrap()).push(
              if                 px0 == px  {' '
              } else if          px1 == px  {'⎅'
              } else if is_px4_black   (px) {'█'
              } else if is_px4_blackish(px) {'▇'
              } else if is_px4_grey_d  (px) {'▓'
              } else if is_px4_grey_l  (px) {'▒'
              } else if is_px3_dark    (px) {'▓'
              } else if is_px3_white   (px) {'□'
              } else if is_px3_whiteish(px) {'◻'//▯
              } else if is_px3_light   (px) {'░'
              } else if is_px3_grey    (px) {'▒'
              } else                        {'•'}//◧
            )}
          });
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
        });
      } else if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR { // 4c·8𝑏pc=32𝑏pp BGRα DIB with mask value in alpha bits
        // ■~black □~white •solid color replacement ◧result depends on bg, ⊻XOR (255,255,255,255 inverts colors?)

        𝑐ℕ=4; let 𝑏pc=8; let px_sz = 𝑐ℕ * 𝑏pc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {𝑏pc} 𝑏⁄𝑐 {px_sz} ■sz𝑏 {row_sz_b} row_sz𝑏\n");}
        // scan_line_test = [35];

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(𝑖row, chunk)| {
          if is_s {if φL>=3&&scan_line_test.contains(&𝑖row) {chunk_test.extend_from_slice(chunk);}}
          if is_s {*s.as_deref_mut().unwrap() += "¦";}
          chunk.chunks(  px_sz  ).enumerate().for_each(|(𝑗col, px   )| {
            if px[3] == 𝑐mask_rep || ( //replaced unconditionally
               px[3] == 𝑐mask_xor && !is_px3_black(px)) { //0 is transparent, so nothing drawn, skip it
              if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱	{most𑁱 = 𝑗col};
              if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪	{most𖭪 = 𝑖row};
            }
            if is_s {(*s.as_deref_mut().unwrap()).push(
              if         px[3] == 𝑐mask_rep { // only two mask values↓
                       // if          px0 == px  {' ' // α stores a mask, not color transparency,…
                // } else if          px1 == px  {'⎅' // … so ignore it, only check RGB
                       if is_px3_black   (px) {'█'
                } else if is_px3_blackish(px) {'▇'
                } else if is_px3_dark    (px) {'▓'
                } else if is_px3_white   (px) {'□'
                } else if is_px3_whiteish(px) {'◻'//▯
                } else if is_px3_light   (px) {'░'
                } else if is_px3_grey    (px) {'▒'
                } else                        {'•'}//◧
              } else  if px[3] == 𝑐mask_xor {
                      if is_px3_black(px) {' '
                  } else                  {'◧'}
              } else                      {'ℯ'} //should be invalid as only 2 mask values are allowed
            )}
          });
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
        });
      }
      // todo: replace with unsafe pointer arithmetic?
      // let mut src = chunk.as_ptr() as *const BGRA8;
      // let    stop = src.add(h as usize);
      // while src != stop {src = src.add(1);}
      // }
      let res_box = cur_box {
        ptl:Point {x: most𐎓 as i32, y: most𖭩 as i32},
        pbr:Point {x: most𑁱 as i32, y: most𖭪 as i32},
        hs :Point {x: hot_x       , y: hot_y}};

      if  most𐎓 > most𑁱
       || most𖭩 > most𖭪 {return Err(CursorSizeErr::BoxSzInvalid(res_box)) }

      if is_s { let ss = s.as_deref_mut().unwrap();
        if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME {for (i,v) in scan_line_test.iter().enumerate() {
        let row_csz = ptr_shape.Pitch as usize;
        let r =                         &chunk_test[(i*row_csz)..((i+1)*row_csz)];
        *ss += &format!("№{v} = "); get𝑏_row(r, ss); *ss += &format!("\n"); }
        } else {                                                  for (i,v) in scan_line_test.iter().enumerate() {
        let row_csz = 𝑐ℕ * w_sz;
        *ss += &format!("№{v} = {:?}\n",&chunk_test[(i*row_csz)..((i+1)*row_csz)]);}  }
        *ss += &format!("←{most𐎓}–{most𑁱}→={} ↑{most𖭩}–{most𖭪}↓={} true bounding box (non0 pixels, 0-based coords )\n",
        most𑁱-most𐎓+1, most𖭪-most𖭩+1);
        let mcur𝑡 = if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME  	{CursorColor::Mono
          } else    if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR       	{CursorColor::Colorα
          } else    if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR	{CursorColor::Colorμ
          } else                                                              	{CursorColor::Colorα};
        *ss += &format!("{}\n{}\n\
          {w} {h}  {hot_x} {hot_y}  {}b  {wb} {mcur𝑡} {mcur𝑡:#?}"
          ,"       Hotspot Bytes B Type"
          ," ↔   ↕  x  y   Size  ↔              №𝑐 𝑏⁄𝑐 𝑏⁄𝑝", ptr_buff.len());
      }

      return Ok(res_box)
    },
  }
}
