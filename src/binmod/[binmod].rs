extern crate helperes      as h    ;
extern crate helperes_proc as hproc;
use ::h            	::*; // gets macros :: prefix needed due to proc macro expansion
pub use hproc      	::*; // gets proc macros
pub use ::h::alias 	::*;
pub use ::h::helper	::*;
// use crate::*;

use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;
pub fn print42() -> Result<()> {p!("{}",42)?; Ok(())}

use std::path::PathBuf;
use docpos::*;

#[docpos] pub struct StructyPos { /// "inner" scruct docs
  pub field1       :        String  ,/// pos-doc for `field1` (in regular Rust this would be a doc for `field2_longer`)
  pub field2_longer: Option<String> ,/// pos-doc for `field2_longer`
                                     /// pos-doc for `field2_longer` line 2
                                     ///! pre-doc for `paths` at `field2_longer` (after `///!`)
  pub paths        : Vec   <PathBuf>, // no doc comments allowed here, use `///!` in the previous field
}
