use crate::*;
use bitvec::prelude::{BitSlice,Msb0,};

use std::path::PathBuf;
use docpos::*;
#[docpos] #[derive(PartialEq)] pub enum CursorColor { /// Type of cursor color/mask <br>
  /// Example of mask data for various cursor types:
  /// |Color        | ⋀   | ⋀   | ⊻     | ⊻     | ⋀⊻     | ⋀⊻    |
  /// |-----------  |---- |---- |------ |----   |------- |-----  |
  /// |             | 24𝑏 | 32𝑏 |24𝑏    | 32𝑏   |dxCM    |dxC    |
  /// |█ Black      | •0  | •0  |␠0  α0 |█0 α1₈ |█0  🆭0  |█0 α1₈ |
  /// |□ White      | •0  | •0  |□1₈ α0 |       |□1₈ 🆭0  |□1₈α1₈ |
  /// |¡ Inverted   | ␠1  | ✗   |□1₈ α0 | ✗     |□1₈ 🆭1₈ | ✗     |
  /// |α-Grey       |  ✗  | •0  | ✗     |       |•𝑐  🆭0  |▓0 αAA |
  /// |␠Transparent |     |     |       |       | 0  🆭1₈ | 0 α0  |
  ///
  /// - `•0` means printed output is `•` and underlying mask data is `0`
  ///   - `1₈` is 1𝑏⋅8 times = `0b11111111` = `0xFF` = `255`
  /// - Color is always in the native BGRα or `0xBBGGRRαα` 32𝑏 data format
  /// - `24𝑏`: TrueColor
  ///   - with no `α`-transparency (so `α`-channel is all `0`s)
  ///   - with `α`-channel acting as a 🆭mask to invert screen colors in [CursorColor::Colorμ]
  /// - `32𝑏`: TrueColor +  `α`<br>
  ///   ⊻ mask sometimes stores pure black with non-pure α: `0₃,255`, `0₃,253`, `0₃,253`, depending on an app<br>
  ///  (e.g., Sib Cursor Editor does that while RealWorld Cursor Editor seems to have `255` all the time)
  ///   - does __NOT__ support Inverted colors, [OS limitation](rw-designer.com/forum/1348). To be more precise: the format itself does, but only if it doesn't have real 32𝑏 data with α>0, otherwise `255₃,0α` will behave exactly like it does in a 24𝑏 format - inverting screen colors (and will be detected as Colorμ by DXGI duplication APIs).
  ///
  /// DirectX Duplication interface:
  ///   - `dxC`  `DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR`
  ///   - `dxCM` `DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR`
  ///     - `🆭` is a mask in `α`-channel, replaces α as transparency
  ///     - `α-Grey` would be a regular color with "transparency" "blended", so not actually transparent
  ///
  Mono  	,///   1𝑐   ·1𝑏⁄𝑐= 1𝑏⁄𝑝      DIB, ⋀AND + ⊻XOR 𝑏mask
  Colorμ	,///  (3𝑐+α)·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB, ⋀AND 𝑏mask + 4color 𝑏map
        	 ///! (3𝑐+🆭)·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB, ⋀AND 𝑏mask + 3color 𝑏map + 🆭=0=⋀AND ¦🆭=255=⊻XOR 𝑏mask  <br>
        	 ///  🆭 0: RGB value replaces the screen pixel  <br>
        	 ///  🆭FF: ⊻XOR is performed on the RGB value and the screen pixel to replace it
  Colorα	,
}
use std::fmt; //{disp} {dbg:?} {disp_alt:#} {dbg_alt:?#}
impl fmt::Display for CursorColor {fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
  if !f.alternate() { let _ =    write!(f,"🖰 𝐶:"); match self {
    CursorColor::Mono  	=> {write!(f,"𝟙" )},
    CursorColor::Colorα	=> {write!(f,"𝟛α")},
    CursorColor::Colorμ	=> {write!(f,"𝟛🆭")},   }
  } else /*#*/      { let _ =     write!(f,"🖰 𝐶:"); match self {
    CursorColor::Mono  	=> {write!(f,"Mono"  )},
    CursorColor::Colorα	=> {write!(f,"All"  )},
    CursorColor::Colorμ	=> {write!(f,"Masked")},   }
}}   }
impl fmt::Debug   for CursorColor {fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
  if !f.alternate() {let _ = fmt::write(f,format_args!("{}::",type_name::<CursorColor>())); match self {
    CursorColor::Mono  	=> {write!(f,"𝟙" )},
    CursorColor::Colorα	=> {write!(f,"𝟛α")},
    CursorColor::Colorμ	=> {write!(f,"𝟛🆭")},   }
  } else /*?#*/     {                                                                       match self {
    CursorColor::Mono  	=> {write!(f," 1𝑐   ·1𝑏⁄𝑐= 1𝑏⁄𝑝      DIB, ⋀AND + ⊻XOR 𝑏mask"  )},
    CursorColor::Colorα	=> {write!(f,"(3𝑐+α)·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB, ⋀AND 𝑏mask + 4color 𝑏map"   )},
    CursorColor::Colorμ	=> {write!(f,"(3𝑐+🆭)·8𝑏⁄𝑐=32𝑏⁄𝑝 BGRα DIB, ⋀AND 𝑏mask + 3color 𝑏map + 🆭=0=⋀AND ¦🆭=255=⊻XOR 𝑏mask")},   }
}} }

#[docpos] #[derive(Debug)] pub enum Mask { /// Type of pixel mask with the following (combined) effects:<br>
  /// (`⋀` AND mask, `⊻` OR mask)<br>
  /// |⋀|0|1 |←⊻ |Base    |
  /// |-|-|--|-- |------- |
  /// |0|█|□ |Δ🗘|🖰cursor |
  /// |1| |◧ |≝  |🖵screen|
  /// | |≝|Δ¡|   |        |
  ///
  /// - `█`Black `□`White `␠`Transparent `◧`Inverted
  /// - mask effect on a pixel:
  ///   - `≝` unchanged (`1`⋀AND `0`⊻XOR)
  ///   - `Δ` changed   (`0`⋀AND `1`⊻XOR):
  ///     - `Δ🗘` replaced (⋀AND)
  ///     - `Δ¡` inverted (⊻XOR)
  ///
  /// For example, `0` ⋀AND mask `Δ🗘` replaces the screen pixel with the `0` black cursor pixel (`0 ⋀ x = 0`), which will then either be `≝` unchanged with `0` ⊻XOR or `Δ¡` inverted with `1` ⊻XOR
  And,/// ⋀ AND mask
    ///!  ⊻ XOR mask
  Xor,
}

#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)] #[docpos]
pub struct Point {pub x:i32, pub y:i32,}

#[derive(Copy,Clone,Debug,PartialOrd,PartialEq,Eq,Ord)] #[docpos]
pub struct cur_box { /// 🖰Mouse cursor real bounding box around actualy drawn pixels, not just the containing bitmap rect
  pub ptl:Point ,/// ↖ top-left     corner point coordinates (x,y) in bounding box coordinates (↖ box = 0,0)
  pub pbr:Point ,/// ↘ bottom-right …
                 ///!  position of the cursor's hot spot relative to its top-left pixel
  pub hs :Option<Point> ,
}


use windows_registry::{CURRENT_USER,Result as Res_win};
pub fn get_cursor_reg() -> Res_win<u32> {
  let key_s = r#"software\Microsoft\Accessibility"#;
  let key_reg = CURRENT_USER.options().read().open(key_s)?;
  let val_reg = key_reg.get_u32("CursorSize")?;
  Ok(val_reg)
}


pub fn get_bits   (x:  u8) -> String {
 let mut s = String::new(); for byte in x.to_be_bytes().iter() { s += &format!("{:08b} ", byte);}  s}
pub fn add_bits   (x:  u8 ,mut s:String) {
                            for byte in x.to_be_bytes().iter() { s += &format!("{:08b} ", byte);}}
pub fn get𝑏_row   (r:&[u8],mut s:&mut String){
  for x in r {              for byte in x.to_be_bytes().iter() {*s += &format!("{:08b} ", byte);}  }   }
pub fn print_bits (x:  u8) {for byte in x.to_be_bytes().iter() {        print!("{:08b} ", byte);}}
pub fn print𝑏_row (r:&[u8]){for x in r {print_bits(*x);}}
pub fn print𝑏_slice(r:&BitSlice<u8,Msb0>){for x in r {print!("{}",if *x{1}else{0});}}


pub const px0: [u8;4] = [0,0,0,0];
pub const px1: [u8;4] = [255,255,255,255];
pub const px_1: [u8;4] = [254,254,254,254];
pub const 𝑐mask_rep:u8 =   0; //         RGB value                  replaces screen pixel
pub const 𝑐mask_xor:u8 = 255; // ⊻XOR of RGB value & screen pixel → replaces screen pixel
pub const 𝑐dark    :u8 =  85; //≈ bottom 1/3 of 255
pub const 𝑐light   :u8 = 170; //≈ top    1/3 of 255

// todo: add bounds checks
pub fn is_px4_black   (px:&[u8]) -> bool{px[0]==  0    && px[1]==  0    && px[2]==  0   && px[3]==255}
pub fn is_px4_blackish(px:&[u8]) -> bool{px[0]<   4    && px[1]<   4    && px[2]<   4   && px[3]==255}
pub fn is_px3_black   (px:&[u8]) -> bool{px[0]==  0    && px[1]==  0    && px[2]==  0   }
pub fn is_px3_blackish(px:&[u8]) -> bool{px[0]<   4    && px[1]<   4    && px[2]<   4   }
pub fn is_px3_white   (px:&[u8]) -> bool{px[0]==255    && px[1]==255    && px[2]==255   }
pub fn is_px3_whiteish(px:&[u8]) -> bool{px[0]> 252    && px[1]> 252    && px[2]> 252   }
pub fn is_px3_dark    (px:&[u8]) -> bool{px[0]< 𝑐dark  && px[1]< 𝑐dark  && px[2]< 𝑐dark }
pub fn is_px3_light   (px:&[u8]) -> bool{px[0]> 𝑐light && px[1]> 𝑐light && px[2]> 𝑐light}
pub fn is_px4_grey_d  (px:&[u8]) -> bool{px[0]==0      && px[1]==0      && px[2]==0     && px[3]< 𝑐dark }
pub fn is_px4_grey_l  (px:&[u8]) -> bool{px[0]==0      && px[1]==0      && px[2]==0     && px[3]> 𝑐light}
pub fn is_px3_grey    (px:&[u8]) -> bool{px[0]==px[1]  && px[1]==px[2]}


// println! conditionally depending on φL level
const φL:u8 = 3;
#[macro_export] macro_rules! φ {($($tokens:tt)*) => {if cfg!(debug_assertions){          pp!("{}",format!($($tokens)*))         } else{} }}
#[macro_export] macro_rules! φ1{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=1{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ2{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=2{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ3{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=3{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ4{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=4{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ5{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=5{pp!("{}",format!($($tokens)*))} else {}} else{} }}
use φ as φ0;

