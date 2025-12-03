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
  /* BITMAP:
    bmType:i32=0   bmPlanes:u16=№color planes (❗NOT colors)
    bmWidth ¦ bmHeight	:i32        	>0 pixels
    bmWidthBytes      	:i32        	№𝑏⁄line, must be EVEN as OS assumes that bit values of a 𝑏map form an array that is word aligned
    bmBitsPixel       	:u16        	𝑏⁄𝑝
    bmBits            	:*mut c_void	ptr to 𝑏map bits'. Its member must be a pointer to an array of character (1-byte) values. ❗null for cursors, use another API to get actual bits*/
  // Store non-empty pixels closest to each of the 4 sides to get the cursor bounding box
    // !: empty cursor will have nonsensical →0 < ←w, this is not checked    ■•◧□
  let mut most𐎓	= usize::MAX; //pushed ← if a valid pixel found
  let mut most𑁱	= 0usize    ; //pushed → …
  let mut most𖭩	= usize::MAX;
  let mut most𖭪	= 0usize    ;

  match cur𝑡 { // Iterate over mouse cursor 𝑏map buffer to detect blank pixels and bounding box size
  CursorColor::Mono      => { let 𝑐ℕ = 1;  // 1𝑐·1𝑏⁄𝑐= 1𝑏⁄𝑝, 𝑏mask has both ⋀AND and ⊻XOR masks
    let mut bmAX = BITMAP::default();
    let bmAXsz = unsafe{ GetObjectW(𝑏mask.into(), size_of::<BITMAP>() as _, Some(&mut bmAX as *mut BITMAP as _)) };
    if  bmAXsz <= 0 {return None}; // no bytes for the buffer. todo: convert to a proper error

    let w  	= bmAX.bmWidth     	; let w_sz  	= w        as usize;
    let wb 	= bmAX.bmWidthBytes	; let row_sz	= wb       as usize; // aka stride
    let h  	= bmAX.bmHeight    	; let h_sz  	=(h / 2)   as usize; // ❗ split between ⋀AND and ⊻XOR masks ❗
    let 𝑏pp	= bmAX.bmBitsPixel 	; let px_sz𝑏	= 𝑏pp      as usize; let px_sz = (𝑏pp / 8) as usize;
    let 𝑏pc	= 𝑏pp / 𝑐ℕ;

    let buf_sz = (wb * h) as usize;

    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{w} ↕{h_sz} ↔{wb}B  {cur𝑡:?}   {𝑐ℕ} №𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz} ■sz (DIB ⋀AND mask + ⊻XOR mask)\n");    }
    let mut cur_buf = vec![0u8; buf_sz];
    let ret = unsafe{GetBitmapBits(𝑏mask, cur_buf.len() as i32, cur_buf.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return None}; // no bytes copied. todo: convert into a proper error

    // 1. Print each mask separately, do box calculations later with both masks applied
    let pad = if h_sz <= 9 {1} else if h_sz <= 99 {2} else {3};
    if is_s {
    cur_buf .chunks(row_sz).enumerate().for_each(|(𝑖row, row)| {let row𝑏 = BitSlice::<_,Msb0>::from_slice(&row);
      (    *s.as_deref_mut().unwrap()).push('¦');
      let 𝑖row0 = if 𝑖row < h_sz {𝑖row} else {𝑖row - h_sz}; // reset 𝑖row to begin from 0 for the 2nd half
      if 𝑖row < h_sz {if 𝑖row==0    {*s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask 1= 0Δ• ———¦\n¦";}
        row𝑏.chunks(px_sz𝑏).enumerate().for_each(|(𝑗col, px )| { // px:&BitSlice<u8>, conceptually [bool] slice
          (*s.as_deref_mut().unwrap()).push(if !px[0] {'•'}else{' '})}        );//Δ AND
      } else         {if 𝑖row==h_sz {*s.as_deref_mut().unwrap() += "——— ⊻XOR Mono◧ bitmask 0= 1Δ• ———¦\n¦";}
        row𝑏.chunks(px_sz𝑏).enumerate().for_each(|(𝑗col, px )| {
          (*s.as_deref_mut().unwrap()).push(if  px[0] {'•'}else{' '})        });//Δ XOR
      }    *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row0:>pad$}\n",pad=pad);
    });   }

    /* 2. Iterate over rows/pixels (px=1𝑏, so iterate BitSlice), calc bound box for ■□◧affected pixels
      ⋀ 0 1 |←⊻	|Base
      0|■ □ |Δ🗘	|🖰cursor
      1|  ◧ |= 	|🖵Screen  only skip ⋀1⊻0 transparent
        =|Δ¡|🖵  */
    if   is_s { *s.as_deref_mut().unwrap() += "¦——— ⋀AND + ⊻XOR Mono◧ bitmask 00•=■black 01••□white 11=•◧inverted🖵 ␠transparent🖵 ———¦\n";}
    for   𝑖row in 0..h_sz { // mask is doubled, and we need to access both to determine pixel state
      if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      let begA = (wb as usize) *  𝑖row        ; let endA = begA + row_sz;
      let begX = (wb as usize) * (𝑖row + h_sz); let endX = begX + row_sz;
      let rowA = &cur_buf[begA..endA]; let rowA𝑏 = BitSlice::<_,Msb0>::from_slice(&rowA);
      let rowX = &cur_buf[begX..endX]; let rowX𝑏 = BitSlice::<_,Msb0>::from_slice(&rowX);

      for 𝑗col in 0..w_sz {
        let pxA = &rowA𝑏[𝑗col..(𝑗col+1)];
        let pxX = &rowX𝑏[𝑗col..(𝑗col+1)];
        let is_draw =
          if        !pxA[0] && !pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('■')}; true
          } else if !pxA[0] &&  pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('□')}; true
          } else if  pxA[0] && !pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push(' ')}; false //🖵 transparent
          } else if  pxA[0] &&  pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('◧')}; true  //🖵◧ Screen reverse/invert
          } else {false}; // should be impossible todo: error here
          if is_draw {if 𝑗col < most𐎓	{most𐎓 = 𝑗col;} if 𝑗col > most𑁱 {most𑁱 = 𝑗col;}
            /**/      if 𝑖row < most𖭩	{most𖭩 = 𝑖row;} if 𝑖row > most𖭪 {most𖭪 = 𝑖row;}  }
      } if is_s { *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
    }
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
    most𑁱 - most𐎓 + 1, most𖭪 - most𖭩 + 1);}

  return Some(cur_box{
    ptl:Point {x: most𐎓 as i32, y: most𖭩 as i32},
    pbr:Point {x: most𑁱 as i32, y: most𖭪 as i32},
    hs :None, })
}
