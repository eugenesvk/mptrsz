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

use std::error::Error;
use std::result;

// type Result<T> = result::Result<T, Box<dyn Error>>;
// fn main() -> Result<()> {
//   print42()?;
//   get_mptr_sz();
//   ret42();
//   Ok(())
// }

// TODO:
  // !!! remove screen capture, only capture the pointer
    // detect which monitor has pointer?
  // move code to lib
  // use argument string for passing out information that's currently printed to stdout
  //trick Rust into using useful simbols: ⵦ𐌭𐎓𐏓𐏔 𑁱𒐀𐎚𒐀 𐋇𐌮ⵃ𒾦𒾠 𐎂𐏑𐏒𒑖𐊵𐊜𐌞 𖣫𖭩𖭪𖭫𖭬𐅁𐅀 too wide? weird spacing𐺉𐺆𐺁 𐺣 a weird space𐳄𐳅  𐤹𐤿 𐱀  no font 𐥉𐥗𐥑
  // let sym_ⵦ𐌭𐎓__𑁱𒐀𐎚𒐀__𐋇𐊁𐌮ⵃ𒾦𒾠___𐎂𐏑𐏒𒑖𐊵𐊜𐌞___𖣫𖭩𖭪𖭫𖭬 = true;𐏐
  // let sym_bad_syntax_highlight___𐏔𐏓__𑁱 = true; 𐄽


use rusty_duplication::{FrameInfoExt, Scanner, VecCapturer, Monitor};
use std::{fs::File, io::Write, thread, time::Duration};
use bitvec::prelude::*; // to iterate over individual pixels packed in a byte
//use bitvec::prelude as 𝑏; // to iterate over individual pixels packed in a byte


/// Color represented by additive channels: Blue (b), Green (g), Red (r), and Alpha (a)
  // DXGI provides a surface that contains a current desktop image through the new IDXGIOutputDuplication::AcquireNextFrame method. The format of the desktop image is always DXGI_FORMAT_B8G8R8A8_UNORM no matter what the current display mode is
  // https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/desktop-dup-api
  // DXGI_FORMAT_B8G8R8A8_UNORM Value:87  A four-component, 32-bit unsigned-normalized-integer format that supports 8 bits for each color channel and 8-bit alpha
#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)]
pub struct BGRA8 {pub b:u8,  pub g:u8,  pub r:u8,  pub a:u8,}

const px0: [u8;4] = [0,0,0,0];
const 𝑐mask_rep:u8 =   0; // RGB value should replace screen pixel
const 𝑐mask_xor:u8 = 255; // ⊻XOR is performed on RGB value and screen pixel; result replaces screen pixel
const 𝑐dark    :u8 =  50;
const 𝑐light   :u8 = 150;

use std::mem;

use windows::{
  Win32::Graphics::{
    Dxgi::{
      Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC, DXGI_MODE_ROTATION_ROTATE90,DXGI_MODE_ROTATION_ROTATE270,},
      DXGI_OUTDUPL_POINTER_SHAPE_TYPE,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR,DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME,
    },
  },
};

use std::path::PathBuf;
use docpos::*;

#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)] #[docpos]
pub struct Point {pub x:i32, pub y:i32,}

#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)] #[docpos]
pub struct mptr_box { /// 🖰Mouse pointer real bounding box around actualy drawn pixels, not just the containing bitmap rect
  pub ptl:Point ,/// ↖ top-left     corner point coordinates (x,y) in bounding box coordinates (↖ box = 0,0)
                 ///!↘ bottom-right …
  pub pbr:Point ,
  // pub hs :Point ,
}
#[docpos] pub struct StructyPos { /// "inner" scruct docs
  pub field1       :        String  ,/// pos-doc for `field1` (in regular Rust this would be a doc for `field2_longer`)
  pub field2_longer: Option<String> ,/// pos-doc for `field2_longer`
                                     /// pos-doc for `field2_longer` line 2
                                     ///! pre-doc for `paths` at `field2_longer` (after `///!`)
  pub paths        : Vec   <PathBuf>, // no doc comments allowed here, use `///!` in the previous field
}


fn main() {
  let mut out_str = String::new();
  let x = 1; //NO: ⹙𜸘🯬˲
  // let xՙ = 1;
  let coords = main_lib(Some(&mut out_str));
  if coords.is_none() {println!("not maybe_ptr_shape{x}");}
  println!("{}",out_str);
}

fn append_to_string(maybe_string: Option<&mut String>) {
  if let Some(s) = maybe_string {
    s.push('1');
  }
}
pub fn is_px3_black(px: &[u8]) -> bool{
  if   px[0] == 0
    && px[1] == 0
    && px[2] == 0  {true} else {false}
}
pub fn is_px3_dark(px: &[u8]) -> bool{
  if   px[0] < 𝑐dark
    && px[1] < 𝑐dark
    && px[2] < 𝑐dark  {true} else {false}
}
pub fn is_px3_light(px: &[u8]) -> bool{
  if   px[0] > 𝑐light
    && px[1] > 𝑐light
    && px[2] > 𝑐light  {true} else {false}
}

fn main_lib(mut s:Option<&mut String>) -> Option<mptr_box> {
  let is_s = s.is_some(); //store a printout string of non-empty pixels

  let mut mon_scanner         	= Scanner::new()    .unwrap(); // Scanner to scan for monitors
  let     monitor :Monitor    	= mon_scanner.next().unwrap(); // Scanner has Iterator, so iterate through monitors
  let mut capturer:VecCapturer	= monitor.try_into().unwrap(); // Create a vec capturer for a monitor this will allocate memory buffer to store pixel data
  // let output_desc  = capturer.monitor().dxgi_output_desc().unwrap(); // you can also get monitor info from a capturer

  // thread::sleep(Duration::from_millis(50)); // sleep before capture to wait system to update the screen
  let capt = capturer.capture_with_pointer_shape().unwrap(); // Res<(DXGI_OUTDUPL_FRAME_INFO,Option<DXGI_OUTDUPL_POINTER_SHAPE_INFO>,)>
  let ptr_buff = capturer.pointer_shape_buffer;

  let maybe_ptr_shape = capt.1;
  match maybe_ptr_shape {None=>{return None},
    Some(ptr_shape)	=> {
      let w = ptr_shape.Width;
      let h = ptr_shape.Height;
      let wb= ptr_shape.Pitch; //🡘b width in bytes of the mouse cursor
      let hot_x = ptr_shape.HotSpot.x; //super::super::Foundation::POINT,
      let hot_y = ptr_shape.HotSpot.y;
        // position of the cursor's hot spot relative to its upper-left pixel
        // app doesn't use hot spot when it determines where to draw the cursor shape
      let ps_type = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(ptr_shape.Type as i32);
      let ptype = match ps_type {
        DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME  	=> "MonoChrome   (1𝑐·1𝑏⁄𝑐= 1𝑏⁄𝑝 DIB ⋀AND mask + ⊻XOR mask)",
        DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR       	=> "Color        (4𝑐·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB)",
        DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR	=> "Masked_Color (4𝑐·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB) with mask value @α bits",
        _                                           	=> "?",
        // only two mask values:
          //    0: RGB value should replace screen pixel
          // 0xFF: ⊻XOR is performed on RGB value and screen pixel; result replaces the screen pixel
      };
      println!("{}\n{}\n\
        {w} {h}  {hot_x} {hot_y}  {}b  {wb}  {ptype}"
        ,"       Hotspot Bytes B Type"
        ," ↔   ↕  x  y   Size  ↔              №𝑐 𝑏⁄𝑐 𝑏⁄𝑝"
        ,ptr_buff.len());


      let mut scan_line_test     = 0;
      let mut chunk_test:Vec<u8> = vec![];
      // !: empty pointer will have nonsensical →0 < ←w, this is not checked    ■•◧□
      let mut most𐎓	= w as usize; //pushed ← if a valid pixel found
      let mut most𑁱	= 0         ; //pushed → …
      let mut most𖭩	= h as usize;
      let mut most𖭪	= 0;

      // not needed to account for rotation?
        // let scan_lines = match output_desc.Rotation {
        //     DXGI_MODE_ROTATION_ROTATE90 |
        //     DXGI_MODE_ROTATION_ROTATE270  => ptr_shape.Width,
        //     _                             => ptr_shape.Height,
        //   }; //  DXGI_MODE_ROTATION_ …  UNSPECIFIED=0  IDENTITY=1  ROTATE90=2  ROTATE180=3  ROTATE270=4
        // println!("{:?} Rotation",output_desc.Rotation);

      // Iterate over mouse pointer buffer to detect blank pixels and true box size

      if        ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME   { //1c·1bpc=1bpp DIB ⋀AND mask + ⊻XOR mask (⋅2))
        // ■black □white
        let hmask = (h/2) as usize; // split between ⋀AND and ⊻XOR masks
        let 𝑐ℕ=1; let bpc=1; let px_sz_b = 𝑐ℕ * bpc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {bpc} 𝑏⁄𝑐 {px_sz_b} ■sz𝑏 {row_sz_b} row_sz𝑏 {hmask}hmask\n");}
        scan_line_test = 90;

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(row   , chunk)| {
          if is_s {if row == scan_line_test {chunk_test = chunk.into();}}
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦");}
          let chunk𝑏 = BitSlice::<_,Msb0>::from_slice(&chunk);
          if row < hmask {if row==0     {if is_s {*s.as_deref_mut().unwrap() += "———⋀AND bitmask———";}}
            chunk𝑏.chunks(bpc     ).enumerate().for_each(|(column, px   )| { // px: &BitSlice<u8>
              if   px[0] == false {
                if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
                if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}  }
              if is_s {*s.as_deref_mut().unwrap() += if px[0]==false {"■"}else{" "}}
            });
          } else         {if row==hmask {if is_s {*s.as_deref_mut().unwrap() += "———⊻XOR bitmask———";}}
            let hrow = row - hmask;
            chunk𝑏.chunks(bpc     ).enumerate().for_each(|(column, px   )| { // px: &BitSlice<u8>
              if   px[0] == true {
                if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
                if hrow   < most𖭩	{most𖭩 = hrow  ;} if hrow   > most𖭪	{most𖭪 = hrow  ;}  }
              if is_s {*s.as_deref_mut().unwrap() += if px[0]==true {"■"}else{" "}}
            });
          }
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
        });

      } else if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR        { //4c·8bpc=32bpp BGRα DIB
        // ■~black □~white ◧other color (visually works best for greys)
        let 𝑐ℕ=4; let bpc=8; let px_sz_b = 𝑐ℕ * bpc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {bpc} 𝑏⁄𝑐 {px_sz_b} ■sz𝑏 {row_sz_b} row_sz𝑏\n");}
        scan_line_test = 54;

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(row   , chunk)| {
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦");}
          if is_s {if row == scan_line_test {chunk_test = chunk.into();}}
          chunk.chunks(  px_sz_b).enumerate().for_each(|(column, px   )| {
            if px != px0 {
              if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
              if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}
            }
            if is_s {*s.as_deref_mut().unwrap() +=
              if px == px0               {" "
              } else if is_px3_dark( px) {"■"
              } else if is_px3_light(px) {"□"
              } else                     {"◧"}
            }
          });
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
        });
      } else if ps_type == DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR { // 4c·8bpc=32bpp BGRα DIB with mask value in alpha bits
        // ■~black □~white •solid color replacement ◧result depends on bg, ⊻XOR (255,255,255,255 inverts colors?)

        let 𝑐ℕ=4; let bpc=8; let px_sz_b = 𝑐ℕ * bpc / 8;
        let row_sz_b = ptr_shape.Pitch as usize; // Pitch = 🡘b width in bytes of mouse pointer
        if is_s {*s.as_deref_mut().unwrap() += &format!("{𝑐ℕ} 𝑐ℕ {bpc} 𝑏⁄𝑐 {px_sz_b} ■sz𝑏 {row_sz_b} row_sz𝑏\n");}
        scan_line_test = 35;

        ptr_buff.chunks(row_sz_b).enumerate().for_each(|(row   , chunk)| {
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦");}
          if is_s {if row == scan_line_test {chunk_test = chunk.into();}}
          chunk.chunks(  px_sz_b).enumerate().for_each(|(column, px   )| {
            if px[3] == 0 { //mask    0: RGB value should replace screen px
              if column < most𐎓	{most𐎓 = column;} if column > most𑁱	{most𑁱 = column;}
              if row    < most𖭩	{most𖭩 = row   ;} if row    > most𖭪	{most𖭪 = row   ;}
            }
            if is_s {*s.as_deref_mut().unwrap() +=
              if         px[3] == 𝑐mask_rep { // only two mask values↓
                      if is_px3_dark( px) {"■"
                }else if is_px3_light(px) {"□"
                }else                     {"•"}
              } else  if px[3] == 𝑐mask_xor {
                      if is_px3_black(px) {" "
                  } else                  {"◧"}
              } else                      {"ℯ"} //should be invalid as only 2 mask values are allowed
            }
          });
          if is_s {*s.as_deref_mut().unwrap() += &format!("¦ №{row}\n");}
        });
      }
      // todo: replace with unsafe pointer arithmetic?
      // let mut src = chunk.as_ptr() as *const BGRA8;
      // let    stop = src.add(h as usize);
      // while src != stop {src = src.add(1);}
      // }
      if is_s {*s.as_deref_mut().unwrap() += &format!("№{scan_line_test} = chunk {chunk_test:?}\n");}
      if is_s {*s.as_deref_mut().unwrap() += &format!("←{most𐎓}–{most𑁱}→={} ↑{most𖭩}–{most𖭪}↓={} true bounding box (non0 pixels, 0-based coords)\n",
        most𑁱-most𐎓+1, most𖭪-most𖭩+1);}
    },
  }

  return Some(mptr_box{ptl:Point{x:0,y:0}, pbr:Point{x:0,y:0}})
  // println!("capturer.pointer_shape_buffer len: {}", ptr_buff.len());
  // let _ = pt(&ptr_buff); //alloc::vec::Vec<u8>

  // println!("capturer.pointer_shape_buffer len: {:?}", ptr_buff.len());

  // let curs = capt.pointer_shape_buffer;

  // thread::sleep(Duration::from_millis(100)); // sleep before capture to wait system to update the screen
  // let info = capturer.capture().unwrap()   ; // capture desktop image and get the frame info
  // // we have some extension methods for the frame info
  // if info.desktop_updated      () {println!("captured! desktop updated");}
  // if info.mouse_updated        () {println!("mouse updated!");}
  // if info.pointer_shape_updated() {println!("pointer shape updated!");}

  // // write to a file
  // let mut file = File::create("capture.bin").unwrap();
  // // the buffer is in BGRA32 format
  // file.write_all(&capturer.buffer).unwrap();
}
