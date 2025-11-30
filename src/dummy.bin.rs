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
use dummy_lib::libmod::{ret42,get_mptr_sz};

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

fn main() {
  let mut out_str = String::new();
  let dbg = true;
  let coords = if dbg	{get_mptr_sz(Some(&mut out_str))
  } else             	{get_mptr_sz(None)};
  println!("{}",out_str);
  match coords {
    Some(c)	=> {println!("coords {:?}",c);},
    None   	=> {println!("no mouse pointer shape captured");},
  };
}
