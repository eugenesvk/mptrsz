use crate::*;
use crate::libmod::*;

use std::ffi::c_void;
use std::mem::size_of;

use docpos::docpos;
use bitvec::prelude::{BitSlice,Msb0,};

use windows::Win32::Graphics::Gdi::{BITMAP,HBITMAP,GetObjectW,GetBitmapBits};

#[docpos]
pub fn measure_mcursor_bm( /// Get the true bounding box of a 🖰 cursor that contains all pixels, based off its ⋀AND and ⊻XOR bitmasks from `GetIconInfo`. Accounts for `Settings`→`Accessibility`→`Size` factor by applying it manually since the API only adjusts the nominal 32·32 size by screen dpi, but not by accessibility resize. Though the result can be 1-2 pixels off compared to the actual size/position (based on DX Duplication API results). Also doesn't take cursor shadow into account (unlike DX Duplication).</br>(masks can be of different size depending on the cursor type, e.g., `⋀AND` can be empty with all `1`s to not overwrite any 🖵screen pixels, but `⊻XOR` can be bigger and invert those pixels with `1`s, so still have a visual effect, so the bounding box is based on the actual visual effect, not just single mask size.)
  𝑏mask	: HBITMAP	,/// 🖰Mono       : ⋀AND top + ⊻XOR bottom
    ///	  </br>  	     🖰Colorμ: ⋀AND
    ///	  </br>  	     🖰Colorα     : ✗
  cur𝑐 	: HBITMAP	,/// 🖰Mono       : ✗          (↑in 𝑏mask)
    ///	  </br>  	     🖰Colorμ     : ⊻XOR-masked mixels without transparency
    ///	  </br>  	     🖰Colorα     : replacement pixels with    transparency
  mut hot_p:Point, /// Hotspot coordinates to be adjusted if Accessibility size > 1
  mut s:Option<&mut String>, /// store the text drawing of the cursor and print a few metrics (mostly for debugging)
  /**/               ///! print mask/color values of these rows (for debugging)
  p_rows:&Vec<usize>,
) -> Result<cur_box,CursorSizeErr>  {
  let is_s = s.is_some(); //store a printout string of non-empty pixels
  /* BITMAP:
    bmType:i32=0   bmPlanes:u16=№color planes (❗NOT colors)
    bmWidth ¦ bmHeight	:i32        	>0 pixels
    bmWidthBytes      	:i32        	№𝑏⁄line, must be EVEN as OS assumes that bit values of a 𝑏map form an array that is word aligned
    bmBitsPixel       	:u16        	𝑏⁄𝑝
    bmBits            	:*mut c_void	ptr to 𝑏map bits'. Its member must be a pointer to an array of character (1-byte) values. ❗null for cursors, use another API to get actual bits*/
  // Store non-empty pixels closest to each of the 4 sides to get the cursor bounding box
    // !: empty cursor will have nonsensical →0 < ←w, this is not checked    ■•◧□ █▓░ ⬛■▣▩▦▧  ❏
  let mut most𐎓	= usize::MAX; //pushed ← if a valid pixel found
  let mut most𑁱	= 0usize    ; //pushed → …
  let mut most𖭩	= usize::MAX;
  let mut most𖭪	= 0usize    ;
  let h_accf:f32;
  let h_accΔ:usize;

  let sz_acc = match get_cursor_reg() {
    Ok (sz_acc) 	=> sz_acc,
    Err(e      )	=> {φ!("Couldn't read CursorSize Accessibility multiplier from the registry! The bounding box will be wrong if the cursor size is > 1  ε={}",e); 1},
  };

  // Iterate over mouse cursor 𝑏map buffer to detect blank pixels and bounding box size
  if cur𝑐.is_invalid() { let cur𝑡 = CursorColor::Mono; // 1𝑐·1𝑏⁄𝑐= 1𝑏⁄𝑝, 𝑏mask has both ⋀AND and ⊻XOR masks
    let 𝑐ℕ = 1;
    let mut bmAX = BITMAP::default();
    let bmAXsz = unsafe{ GetObjectW(𝑏mask.into(), size_of::<BITMAP>() as _, Some(&mut bmAX as *mut BITMAP as _)) };
    if  bmAXsz <= 0 {return Err(CursorSizeErr::Bitmap("Mono: ‘GetObjectW’ got no bytes for the 𝑏mask buffer".into()))};

    let w  	= bmAX.bmWidth     	; let w_sz  	= w        as usize;
    let wb 	= bmAX.bmWidthBytes	; let row_sz	= wb       as usize; // aka stride
    let h  	= bmAX.bmHeight    	; let h_sz  	=(h / 2)   as usize; // ❗ split between ⋀AND and ⊻XOR masks ❗
    let 𝑏pp	= bmAX.bmBitsPixel 	; let px_sz𝑏	= 𝑏pp      as usize; let px_sz = (𝑏pp / 8) as usize;
    let 𝑏pc	= 𝑏pp / 𝑐ℕ;

    let buf_sz = (wb * h) as usize;
    h_accΔ = ((sz_acc - 1) as usize) * (h_sz / 2); // 1 unit of accessibilitiy scale increases cursor size by half
    h_accf = 1.0 + (h_accΔ as f32 / h_sz as f32);

    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{w} ↕{h_sz} ↔{wb}B  {cur𝑡:?}   {𝑐ℕ}№𝑐⋅{𝑏pc}𝑏⁄𝑐={𝑏pp}𝑏⁄𝑝 {px_sz}■sz {sz_acc}⋅🮰sz (DIB ⋀AND mask + ⊻XOR mask)\n");    }
    let mut cur_buf = vec![0u8; buf_sz];
    let ret = unsafe{GetBitmapBits(𝑏mask, cur_buf.len() as i32, cur_buf.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return Err(CursorSizeErr::Bitmap("Mono: ‘GetBitmapBits’ copied no bytes from 𝑏mask".into()))};

    // 1. Print each mask separately, do box calculations later with both masks applied
    let pad = if h_sz <= 9 {1} else if h_sz <= 99 {2} else {3};
    if is_s {
    cur_buf .chunks(row_sz).enumerate().for_each(|(𝑖row, row)| {let row𝑏 = BitSlice::<_,Msb0>::from_slice(row);
      (    *s.as_deref_mut().unwrap()).push('¦');
      let 𝑖row0 = if 𝑖row < h_sz {𝑖row} else {𝑖row - h_sz}; // reset 𝑖row to begin from 0 for the 2nd half
      if 𝑖row < h_sz {if 𝑖row==0    {*s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask 1≝ 0Δ• ———¦\n¦";}
        row𝑏.chunks(px_sz𝑏).for_each(|px| { // px:&BitSlice<u8>, conceptually [bool] slice
          (*s.as_deref_mut().unwrap()).push(if !px[0] {'•'}else{' '})}        );//Δ AND
      } else         {if 𝑖row==h_sz {*s.as_deref_mut().unwrap() += "——— ⊻XOR Mono◧ bitmask 0≝ 1Δ• ———¦\n¦";}
        row𝑏.chunks(px_sz𝑏).for_each(|px| {
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
      let rowA = &cur_buf[begA..endA]; let rowA𝑏 = BitSlice::<_,Msb0>::from_slice(rowA);
      let rowX = &cur_buf[begX..endX]; let rowX𝑏 = BitSlice::<_,Msb0>::from_slice(rowX);

      for 𝑗col in 0..w_sz {
        let pxA = &rowA𝑏[𝑗col..(𝑗col+1)];
        let pxX = &rowX𝑏[𝑗col..(𝑗col+1)];
        let is_draw =
          if        !pxA[0] && !pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('■')}; true
          } else if !pxA[0] &&  pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('□')}; true
          } else if  pxA[0] && !pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push(' ')}; false //🖵 transparent
          } else if  pxA[0] &&  pxX[0] {if is_s {(*s.as_deref_mut().unwrap()).push('◧')}; true  //🖵◧ Screen reverse/invert
          } else {false}; // should be impossible todo: error here
          if is_draw {if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱 {most𑁱 = 𝑗col};
            /**/      if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪 {most𖭪 = 𝑖row};  }
      } if is_s { *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
    }
  } else { // 1st check if α is > 0 to detect Colorμ, then parse the 𝑏map buffer (both Colorα and Colorμ are technically 32𝑏⁄𝑝 with Colorμ having α=0)
    // Parse both mono 𝑏mask and color 𝑏map then get image bits to detect cursor type
    let mut bmA = BITMAP::default(); //monochrome 𝑏mask
    let mut bmX = BITMAP::default(); //color      𝑏map
    let bmAsz = unsafe{ GetObjectW(𝑏mask.into(), size_of::<BITMAP>() as _, Some(&mut bmA as *mut BITMAP as _)) };
    let bmXsz = unsafe{ GetObjectW(cur𝑐 .into(), size_of::<BITMAP>() as _, Some(&mut bmX as *mut BITMAP as _)) };
    if  bmAsz <= 0 {return Err(CursorSizeErr::Bitmap("Colorμ: ‘GetObjectW’ copied no bytes for the monochrome 𝑏mask buffer".into()))};
    if  bmXsz <= 0 {return Err(CursorSizeErr::Bitmap("Colorμ: ‘GetObjectW’ copied no bytes for the color 𝑏map buffer".into()))};

    // Monochrome 𝑏mask
    let 𝑐ℕA 	= 1;
    let wA  	= bmA.bmWidth     	; let _wA_sz 	= wA   as usize;
    let wAb 	= bmA.bmWidthBytes	; let rowA_sz	= wAb  as usize; // aka stride
    let hA  	= bmA.bmHeight    	; let _hA_sz 	= hA   as usize;
    let 𝑏ppA	= bmA.bmBitsPixel 	; let pxA_sz𝑏	= 𝑏ppA as usize; let pxA_sz = (𝑏ppA / 8) as usize;
    let 𝑏pcA	= 𝑏ppA / 𝑐ℕA;
    let bufA_sz = (wAb * hA) as usize;

    let mut curA_buf = vec![0u8; bufA_sz];
    let ret = unsafe{GetBitmapBits(𝑏mask, curA_buf.len() as i32, curA_buf.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return Err(CursorSizeErr::Bitmap("Colorμ: ‘GetBitmapBits’ copied no bytes from the monochrome 𝑏mask".into()))};

    // Color bits
    let 𝑐ℕX 	= 4; //unknown whether the 4th color is 0s (masked 24𝑏) before parsing the α channel
    let wX  	= bmX.bmWidth     	; let wX_sz   	= wX   as usize;
    let wXb 	= bmX.bmWidthBytes	; let rowX_sz 	= wXb  as usize; // aka stride
    let hX  	= bmX.bmHeight    	; let hX_sz   	= hX   as usize;
    let 𝑏ppX	= bmX.bmBitsPixel 	; let _pxX_sz𝑏	= 𝑏ppX as usize; let pxX_sz = (𝑏ppX / 8) as usize;
    let 𝑏pcX	= 𝑏ppX / 𝑐ℕX;
    let bufX_sz = (wXb * hX) as usize;
    let 𝑐ℕX_sz	= 𝑐ℕX        as usize;
    h_accΔ = ((sz_acc - 1) as usize) * (hX_sz / 2); // 1 unit of accessibilitiy scale increases cursor size by half
    h_accf = 1.0 + (h_accΔ as f32 / hX_sz as f32);

    let mut curX_buf = vec![0u8; bufX_sz];
    let ret = unsafe{GetBitmapBits(cur𝑐, curX_buf.len() as i32, curX_buf.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return Err(CursorSizeErr::Bitmap("Colorμ: ‘GetBitmapBits’ copied no bytes from the color 𝑏map".into()))};

    let mut isα = false; // Detect α bits
    curX_buf.chunks(rowX_sz).for_each(|row| {
      row   .chunks( pxX_sz).for_each(|px | {
        if px[3] != 0 {isα = true}      });    });

    let _is_colα	=  isα;
    let is_colμ 	= !isα;

  if is_colμ {let cur𝑡 = CursorColor::Colorμ; //4c·8𝑏pc=32𝑏pp BGRα DIB  both 𝑏mask and color 𝑏map
    // 1. Print each mask separately, do box calculations later with both masks applied
    let pad = if hX_sz <= 9 {1} else if hX_sz <= 99 {2} else {3};
    if is_s {
         *s.as_deref_mut().unwrap() += &format!(
      "↔{wA} ↕{hA} ↔{wAb}B  {cur𝑡:?}   {𝑐ℕA}№𝑐⋅{𝑏pcA}𝑏⁄𝑐={𝑏ppA}𝑏⁄𝑝 {pxA_sz}■sz {sz_acc}⋅🮰sz Mono◧ 𝑏mask (BGRα DIB)\n");
         *s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask 1≝ 0Δ• ———¦\n";
    curA_buf.chunks(rowA_sz).enumerate().for_each(|(𝑖row, row)| {let row𝑏 = BitSlice::<_,Msb0>::from_slice(row);
      if φL>=3&&p_rows.contains(&𝑖row){print!("№{𝑖row:>pad$}𝑏= ",pad=pad);print𝑏_row(row);pp!();}
      (  *s.as_deref_mut().unwrap()).push('¦');
      row𝑏  .chunks(pxA_sz𝑏).for_each(|px| { // px:&BitSlice<u8>, conceptually [bool] slice
        (*s.as_deref_mut().unwrap()).push(if !px[0] {'•'}else{' '})}        );//Δ AND
         *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);
    });

         *s.as_deref_mut().unwrap() += &format!(
      "↔{wX} ↕{hX} ↔{wXb}B  {cur𝑡:?}   {𝑐ℕX}№𝑐⋅{𝑏pcX}𝑏⁄𝑐={𝑏ppX}𝑏⁄𝑝 {pxX_sz}■sz {sz_acc}⋅🮰sz Color 𝑏map (BGRα DIB)\n");
         *s.as_deref_mut().unwrap() += "——— ⊻XOR Color bitmap 0≝ 1Δ• ———¦\n";
    curX_buf.chunks(rowX_sz).enumerate().for_each(|(𝑖row, row)| {(*s.as_deref_mut().unwrap()).push('¦');
      if φL>=3&&p_rows.contains(&𝑖row){pp!("№{𝑖row:>pad$} {row:?}",pad=pad);}
      row   .chunks( pxX_sz).for_each(|px| {(*s.as_deref_mut().unwrap()).push(
        if              px0 == px  {' '
        } else if       px1 == px
          ||            px_1== px  {'⎅' // some apps like Sib output 254 instead of all 255
        // } else if       0   == px[3]{'α' //α-transparent, but ■□•mark since XORing with ⋀0 will still result in color changes, same with ⋀1 and screen α
        // todo: compare 24b with 32b and how to deal with the fact that 24b has no alpha
        // is there a guaranteed way to detect 24b? if all α=0
        } else if is_px3_black   (px) {'█'
        } else if is_px3_blackish(px) {'▇'
        } else if is_px3_dark    (px) {'▓'
        } else if is_px3_white   (px) {'□'
        } else if is_px3_whiteish(px) {'◻'//▯
        } else if is_px3_light   (px) {'░'
        } else if is_px3_grey    (px) {'▒'
        } else                     {'•'}) //◧
      });*s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);
    });  }


    /* 2. Iterate over rows/pixels (Mono◧ px=1𝑏, so iterate BitSlice), calc bound box for ■□◧affected pixels
      ⋀ 0 1 |←⊻	|Base
      0|■ □ |Δ🗘	|🖰cursor
      1|  ◧ |= 	|🖵Screen  only skip ⋀1⊻0 transparent
        =|Δ¡|🖵  */
    if   is_s { *s.as_deref_mut().unwrap() += "¦——— ⋀AND Mono◧ 𝑏mask + ⊻XOR Color 0𝑐_••→■dark¦□light¦•other 1𝑐_␠•_◧inverted🖵¦␠transparent🖵¦⊻XORed🖵 ———¦\n";}

    for   𝑖row in 0..hX_sz { // both masks ⋀+⊻ are needed to determine pixel state
      if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      let begA = (wAb as usize) * 𝑖row; let endA = begA + rowA_sz;
      let begX = (wXb as usize) * 𝑖row; let endX = begX + rowX_sz;
      let rowA = &curA_buf[begA..endA]; let rowA𝑏 = BitSlice::<_,Msb0>::from_slice(rowA);
      let rowX = &curX_buf[begX..endX];

      if φL>=4&&p_rows.contains(&𝑖row){//12,13,24,25
      print!("№{𝑖row:>pad$}𝑏= "        ,pad=pad);print𝑏_row(rowA);pp!();
      pp!(   "№{𝑖row:>pad$} = {rowX:?}",pad=pad);}
      for 𝑗col in 0..wX_sz {
        let begA = 𝑗col         ; let endA = begA + (𝑐ℕA as usize);
        let begX = 𝑗col * 𝑐ℕX_sz; let endX = begX + 𝑐ℕX_sz;
        let pxA = &rowA𝑏[begA..endA];
        let pxX = &rowX [begX..endX];
        // if 𝑖row==0 {print!("№{𝑖row:>pad$}𝑏¦№{𝑗col:>pad$} = ",pad=pad);print𝑏_slice(pxA);pp!(" ¦ {pxX:?}");} //todo: delete / uncomment debug print
        let is_draw =
          if        !pxA[0] { //base=🖰cursor px 0█ 1□
            if              px0 == pxX  {if is_s {(*s.as_deref_mut().unwrap()).push('█')}; false
              //α is not transparency, but a flag for RGB=0,0,0'█' to replace screen
            // } else if       0   == pxX[3]{if is_s{(*s.as_deref_mut().unwrap()).push('α')}; true
              //α=0 is a flag to replace with px RGB '•', not α-transparen, but we differentiate shades↓
            } else if is_px3_black   (pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('█')}; true
            } else if is_px3_blackish(pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('▇')}; true
            } else if is_px3_dark    (pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('▓')}; true
            } else if is_px3_white   (pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('□')}; true
            } else if is_px3_whiteish(pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('◻')}; true
            } else if is_px3_light   (pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('░')}; true
            } else if is_px3_grey    (pxX) {if is_s {(*s.as_deref_mut().unwrap()).push('▒')}; true
            } else                         {if is_s {(*s.as_deref_mut().unwrap()).push('•')}; true} //◧
          } else if  pxA[0] { //⋀1→base=🖵screen px   ◧inverted🖵 or ⊻XORed🖵¦␠transparent🖵
            if              px0 == pxX  {if is_s {(*s.as_deref_mut().unwrap()).push(' ')}; false
            } else                      {if is_s {(*s.as_deref_mut().unwrap()).push('◧')}; true}//⊻color mask
          } else {false}; // should be impossible todo: error here
          // pp!("i{𝑖row} j{𝑗col} px={pxX:?}");
          if is_draw {if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱 {most𑁱 = 𝑗col};
            /**/      if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪 {most𖭪 = 𝑖row};  }
      } if is_s { *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
    }
  } else     {let cur𝑡 = CursorColor::Colorα; // 4𝑐·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB, no 𝑏mask → draw color px directly
    let pad = if hX_sz <= 9 {1} else if hX_sz <= 99 {2} else {3};

    if is_s {
         *s.as_deref_mut().unwrap() += &format!(
      "↔{wA} ↕{hA} ↔{wAb}B  {cur𝑡:?}   {𝑐ℕA}№𝑐⋅{𝑏pcA}𝑏⁄𝑐={𝑏ppA}𝑏⁄𝑝 {pxA_sz}■sz {sz_acc}⋅🮰sz {rowA_sz}rowA Mono◧ 𝑏mask (BGRα DIB)\n");
         *s.as_deref_mut().unwrap() += "——— ⋀AND Mono◧ bitmask 1≝ 0Δ• ———¦\n";
         if sz_acc > 1 {
         *s.as_deref_mut().unwrap() += "——— (likely nonsensical since 🮰sz Accessibility Size > 1)¦\n";}
    curA_buf.chunks(rowA_sz).enumerate().for_each(|(𝑖row, row)| {let row𝑏 = BitSlice::<_,Msb0>::from_slice(row);
      if φL>=3&&p_rows.contains(&𝑖row){print!("№{𝑖row:>pad$}𝑏= ",pad=pad);print𝑏_row(row);pp!();}
      (  *s.as_deref_mut().unwrap()).push('¦');
      row𝑏  .chunks(pxA_sz𝑏).for_each(|px| { // px:&BitSlice<u8>, conceptually [bool] slice
        (*s.as_deref_mut().unwrap()).push(if !px[0] {'•'}else{' '})}        );//Δ AND
         *s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);
    });
    }

    if is_s { *s.as_deref_mut().unwrap() += &format!(
      "↔{wX} ↕{hX} ↔{wXb}B  {cur𝑡:?}   {𝑐ℕX}№𝑐⋅{𝑏pcX}𝑏⁄𝑐={𝑏ppX}𝑏⁄𝑝 {pxX_sz} ■sz (BGRα DIB)\n");    }
    let mut cur_buf = vec![0u8; bufX_sz];
    let ret = unsafe{GetBitmapBits(cur𝑐, cur_buf.len() as i32, cur_buf.as_mut_ptr() as *mut c_void,) };
    if  ret == 0 {return Err(CursorSizeErr::Bitmap("Colorα: ‘GetBitmapBits’ copied no bytes from the color 𝑏map".into()))};

    if is_s {*s.as_deref_mut().unwrap() += "——— Color 𝑏map ■dark¦□light¦•other ———\n";} //◧visually works best for greys
    cur_buf.chunks(rowX_sz).enumerate().for_each(|(𝑖row, row)| {if is_s {(*s.as_deref_mut().unwrap()).push('¦');}
      row  .chunks( pxX_sz).enumerate().for_each(|(𝑗col, px )| {
        let is_draw =
          if              px0 == px
            ||              0 == px[3]{if is_s {(*s.as_deref_mut().unwrap()).push(' ')};false//transparency also affects RGB, so it's 15,15,15,15 or with α=0 would be px0, so this should be redundant? No, can be forced to have 255,255,255,0 in an app for 'inverted' color that has no effect in a non-masked format
          } else if is_px3_dark (px)  {if is_s {(*s.as_deref_mut().unwrap()).push('▓')};true//■
          } else if is_px3_light(px)  {if is_s {(*s.as_deref_mut().unwrap()).push('░')};true//❏
          } else                      {if is_s {(*s.as_deref_mut().unwrap()).push('•')};true};//◧
        if is_draw {if 𝑗col < most𐎓	{most𐎓 = 𝑗col}; if 𝑗col > most𑁱 {most𑁱 = 𝑗col};
            /**/    if 𝑖row < most𖭩	{most𖭩 = 𝑖row}; if 𝑖row > most𖭪 {most𖭪 = 𝑖row};  }
      });if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{𝑖row:>pad$}\n",pad=pad);}
    });
  }
  }
  // todo: replace with unsafe pointer arithmetic? to avoid bound checks??
  // let mut src = row.as_ptr() as *const BGRA8;
  // let    stop = src.add(h as usize);
  // while src != stop {src = src.add(1);}
  // }

  if  most𐎓 > most𑁱 // todo: convert to proper error
   || most𖭩 > most𖭪 {return Err(CursorSizeErr::Ii("bounding box is invalid, is the cursor blank?".into()))};

  if sz_acc > 1 { // adjust bounding box bottom/right sides by accessibility Δ since GetCursorInfo retrieves cursor mask of the default size (only adjusted by screen scaling, so 32⋅32⋅dpi)
  if is_s {*s.as_deref_mut().unwrap() += &format!(
    "←{most𐎓}–{most𑁱}→={} ↑{most𖭩}–{most𖭪}↓={} bound box PRE accessibility scaling (⋅{}) HS•x{} y{}\n",
    most𑁱 - most𐎓 + 1, most𖭪 - most𖭩 + 1, h_accf, hot_p.x, hot_p.y);}
    most𖭩 = (most𖭩 as f32 * h_accf).round() as usize;
    most𐎓 = (most𐎓 as f32 * h_accf).round() as usize;
    most𑁱 = (most𑁱 as f32 * h_accf).round() as usize;
    most𖭪 = (most𖭪 as f32 * h_accf).round() as usize;

    hot_p.x = (hot_p.x as f32 * h_accf).round() as i32;
    hot_p.y = (hot_p.y as f32 * h_accf).round() as i32;
  }

  if is_s {*s.unwrap() += &format!(
    "←{most𐎓}–{most𑁱}→={} ↑{most𖭩}–{most𖭪}↓={} bound box (¬0 px, 0-based coords) HS•x{} y{}\n",
    most𑁱 - most𐎓 + 1, most𖭪 - most𖭩 + 1, hot_p.x, hot_p.y);}

  Ok(cur_box{
    ptl:Point {x: most𐎓 as i32, y: most𖭩 as i32},
    pbr:Point {x: most𑁱 as i32, y: most𖭪 as i32},
    hs :hot_p, })
}
