#![cfg_attr(not(debug_assertions),allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types,confusable_idents))]
#![cfg_attr(    debug_assertions ,allow(uncommon_codepoints,non_snake_case,non_upper_case_globals,non_camel_case_types,confusable_idents,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;

#[path="libmod/[libmod].rs"] pub mod libmod;
use crate::libmod::{ret42,get_mptr_sz};

pub fn lib() -> i32 {
  let coords = get_mptr_sz(None);
  // match coords {
    // Some(c)	=> {println!("coords {:?}",c);},
    // None   	=> {println!("no mouse pointer shape captured");},
  // };
  ret42()
}
