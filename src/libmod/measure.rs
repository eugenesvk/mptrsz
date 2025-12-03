use bitvec::prelude::{BitSlice,Msb0,};
use crate::libmod::*;
use docpos::docpos;
use std::slice;
use std::ffi::c_void;
use std::mem::{self,size_of,zeroed};
use windows::Win32::{
  Foundation::{POINT,BOOL,TRUE,FALSE,},
  Graphics::Gdi::{BITMAP,HGDIOBJ,HBITMAP,
    DeleteObject,GetObjectW,GetBitmapBits,GetDIBits},
  UI::WindowsAndMessaging::{HICON,ICONINFO,CURSORINFO,HCURSOR,CURSORINFO_FLAGS,CURSOR_SHOWING,CURSOR_SUPPRESSED,
  GetCursor,GetCursorPos,GetCursorInfo,GetIconInfo}
};

#[docpos]
pub fn measure_mcursor_bm( /// Get the true bounding box of a 🖰 cursor that contains all pixels, based off its ⋀AND and ⊻XOR bitmasks from GetIconInfo.</br>Masks can be of different size depending on the desired effect, e.g., ⋀AND can be empty with all 1s to not overwrite any 🖵pixels, but ⊻XOR can be bigger and invert those pixels with 1s, so still have a visual effect, so the bounding box should be the bigger of the two: 0 ⋀AND.
  𝑏mask	: HBITMAP    	,/// 🖰Mono       : ⋀AND top + ⊻XOR bottom
    ///	  </br>      	     🖰ColorMasked: ⋀AND
    ///	  </br>      	     🖰Color      : ✗
  cur𝑐 	: HBITMAP    	,/// 🖰Mono       : ✗          (↑in 𝑏mask)
    ///	  </br>      	     🖰ColorMasked: ⊻XOR
    ///	  </br>      	     🖰Color      : replacement pixels (?todo)
  cur𝑡 	:&CursorColor	,/// 🖰Type, affects whether 𝑏mask also contains ⊻XOR
  ///! store the text drawing of the cursor and print a few metrics (mostly for debugging)
  mut s:Option<&mut String>
) -> Option<cur_box>  {
  let is_s = s.is_some(); //store a printout string of non-empty pixels

  let mut bmA = BITMAP::default();
  let mut bmX = BITMAP::default();
    // bmType:i32=0   bmPlanes:u16=№color planes (NOT!!! colors)
    // bmWidth ¦ bmHeight	:i32        	// >0 pixels
    // bmWidthBytes      	:i32        	// №𝑏⁄line, must be EVEN as OS assumes that bit values of a bitmap form an array that is word aligned
    // bmBitsPixel       	:u16        	// 𝑏⁄𝑝
    // bmBits            	:*mut c_void	// ptr to bitmap bits'. Its member must be a pointer to an array of character (1-byte) values. ❗null for cursors, use another API to get actual bits
  let bmAsz = unsafe{ GetObjectW(maskA.into(), size_of::<BITMAP>() as _, Some(&mut bmA as *mut BITMAP as _)) };
  let bmXsz = unsafe{ GetObjectW(maskX.into(), size_of::<BITMAP>() as _, Some(&mut bmX as *mut BITMAP as _)) };
  match *cur_col { //todo: convert into errors
    CursorColor::Mono       	=> {if  bmAsz <= 0 {return None}}, //mono cursors have all info in AND
    CursorColor::Color      	=> {if  bmXsz <= 0 {return None}}, //color only have XOR
    CursorColor::ColorMasked	=> {if (bmAsz <= 0) || bmXsz <= 0 {return None}}, //masked have both
  }

  let w  	= bmX.bmWidth ;
  let wb 	= bmX.bmWidthBytes; //🡘b width in bytes of the mouse cursor aka stride
  let h  	= bmX.bmHeight; // !2 AND+XOR masks
  let 𝑏pp	= bmX.bmBitsPixel;
  let px_sz𝑏 = 𝑏pp      as usize;
  let px_sz = (𝑏pp / 8) as usize;
  let row_sz = wb       as usize;
  let buf_sz =(wb * h ) as usize;

  let w_sz = w as usize;
  let h_sz = h as usize;
  let stride = wb as usize;

  // Store non-empty pixels closest to each of the 4 sides to get the cursor bounding box
  // !: empty cursor will have nonsensical →0 < ←w, this is not checked    ■•◧□
  let mut most𐎓	= w as usize; //pushed ← if a valid pixel found
  let mut most𑁱	= 0         ; //pushed → …
  let mut most𖭩	= h as usize;
  let mut most𖭪	= 0;


  match cur_col { // Iterate over mouse cursor bitmap buffer to detect blank pixels and bounding box size
  CursorColor::Mono      => {let 𝑐ℕ = 1; let 𝑏pc = 𝑏pp / 𝑐ℕ; //1c·1𝑏pc=1𝑏pp
    // ■black □white
    let hm = (h/2) as usize; // split between ⋀AND and ⊻XOR masks
    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{w} ↕{hm} ↔{wb}B  {cur_col:?}   {𝑐ℕ} №𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz} ■sz (DIB ⋀AND mask + ⊻XOR mask)\n");    }
    let mut ptr_buff = vec![0u8; buf_sz];
    let ret = unsafe{GetBitmapBits(maskA, ptr_buff.len() as i32, ptr_buff.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return None}; //todo: convert into a proper error

    // todo: why was it bmAsz ???
    // let ptr_buff = unsafe{slice::from_raw_parts(bmX.bmBits as *const u8, bmAsz as usize)}; //№of el, not bytes, but in this case colors don't align, so just use bytes, but in this case we can't fit colors into els

    ptr_buff.chunks(  row_sz).enumerate().for_each(|(row   , chunk)| {let chunk𝑏 = BitSlice::<_,Msb0>::from_slice(&chunk);
      if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      if row < hm {if row==0  {if is_s {*s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask ———¦\n¦";}}
        chunk𝑏.chunks(px_sz𝑏).enumerate().for_each(|(column, px   )| { // px: &BitSlice<u8>
          if  !px[0] {if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
            /**/      if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}  }
          if is_s {(*s.as_deref_mut().unwrap()).push(if !px[0] {'■'}else{' '})}        });
      } else      {if row==hm {if is_s {*s.as_deref_mut().unwrap() += "——— ⊻XOR Mono◧ bitmask ———¦\n¦";}}
        let hrow = row - hm; //reset row index to begin from 0 for the 2nd half
        chunk𝑏.chunks(px_sz𝑏).enumerate().for_each(|(column, px   )| { // px: &BitSlice<u8>
          if   px[0] {if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
            /**/      if hrow   < most𖭩	{most𖭩 = hrow  ;} if hrow   > most𖭪	{most𖭪 = hrow  ;}  }
          if is_s {(*s.as_deref_mut().unwrap()).push(if  px[0] {'■'}else{' '})}        });
      }   if is_s { *s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
    });
  },
  CursorColor::Color     => {let 𝑐ℕ = 4; let 𝑏pc = 𝑏pp / 𝑐ℕ; //4c·8𝑏pc=32𝑏pp BGRα DIB
    // ■~black □~white ◧other color (visually works best for greys)

    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{w} ↕{h} ↔{wb}B  {cur_col:?}   {𝑐ℕ} №𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz} ■sz (BGRα DIB)\n");    }
    let mut ptr_buff = vec![0u8; buf_sz];
    let ret = unsafe{GetBitmapBits(maskA, ptr_buff.len() as i32, ptr_buff.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return None}; //todo: convert into a proper error

    if is_s {*s.as_deref_mut().unwrap() += "——— ⊻XOR Color bitmap ———\n";}
    ptr_buff.chunks(row_sz).enumerate().for_each(|(row   , chunk)| {
      if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      chunk.chunks(  px_sz).enumerate().for_each(|(column, px   )| {
        if px != px0 {if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
          /**/        if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}  }
        if is_s {(*s.as_deref_mut().unwrap()).push(
          if              px0 == px  {' '
          } else if is_px3_dark (px) {'■'
          } else if is_px3_light(px) {'□'
          } else                     {'◧'})}
      });if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
    });
  },
  // TODO: what about the monochrome mask for masked color
  CursorColor::ColorMasked => {let 𝑐ℕ = 4; let 𝑏pc = 𝑏pp / 𝑐ℕ; //4c·8𝑏pc=32𝑏pp BGRα DIB with mask value in alpha bits
    // ■~black □~white •solid color replacement ◧result depends on bg, ⊻XOR (255,255,255,255 inverts colors?)
    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{w} ↕{h} ↔{wb}B  {cur_col:?}   {𝑐ℕ} №𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz} ■sz (BGRα DIB)\n");    }
    println!("↔{w} ↕{h} ↔{wb}B  {cur_col:?}   {𝑐ℕ} №𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz} ■sz (BGRα DIB)\n");
    let mut ptr_buff = vec![0u8; buf_sz];
    let ret = unsafe{GetBitmapBits(maskA, ptr_buff.len() as i32, ptr_buff.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return None}; //todo: convert into a proper error

    if 𝑏pp == 1 {if is_s {*s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask ———\n";}
    ptr_buff.chunks(  row_sz).enumerate().for_each(|(row   , chunk)| {let chunk𝑏 = BitSlice::<_,Msb0>::from_slice(&chunk);
      if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      chunk𝑏.chunks(px_sz𝑏).enumerate().for_each(|(column, px   )| { // px: &BitSlice<u8>
        if is_s {(*s.as_deref_mut().unwrap()).push(if !px[0] {'■'}else{' '})}        });
        if is_s { *s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
    }); return None} else {if is_s {*s.as_deref_mut().unwrap() += "——— ⊻XOR Color bitmap ———\n";}
    ptr_buff.chunks(row_sz).enumerate().for_each(|(row   , chunk)| {if is_s{(*s.as_deref_mut().unwrap()).push('¦');}
      chunk.chunks(  px_sz).enumerate().for_each(|(column, px   )| {
        if px[3] == 𝑐mask_rep {if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
          /**/                 if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}  }
        if is_s {(*s.as_deref_mut().unwrap()).push(
          if         px[3] == 𝑐mask_rep { // only two mask values↓
                  if is_px3_dark (px) {'■'
            }else if is_px3_light(px) {'□'
            }else                     {'•'}
          } else  if px[3] == 𝑐mask_xor {
                  if is_px3_black(px) {' '
            }else                     {'◧'}
          } else                      {'ℯ'}) } //invalid as only 2 mask values are allowed
      });if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
    });  }
  },   }
    // todo: replace with unsafe pointer arithmetic?
    // let mut src = chunk.as_ptr() as *const BGRA8;
    // let    stop = src.add(h as usize);
    // while src != stop {src = src.add(1);}
    // }
  if is_s {*s.as_deref_mut().unwrap() += &format!(
    "←{most𐎓}–{most𑁱}→={} ↑{most𖭩}–{most𖭪}↓={} bound box (¬0 px, 0-based coords)\n",
    most𑁱-most𐎓+1, most𖭪-most𖭩+1);}

  return Some(mptr_box{
    ptl:Point {x: most𐎓 as i32, y: most𖭩 as i32},
    pbr:Point {x: most𑁱 as i32, y: most𖭪 as i32},
    hs :Point {x:0,y:0}})
}
