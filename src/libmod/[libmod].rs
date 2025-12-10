mod error;
mod ext;
mod capture;
mod measure;

mod error_test;
mod capture_test;

pub use error::*;
pub use capture::*;
pub use measure::*;
pub use ext::*;

// println! conditionally depending on φL level
const φL:u8 = 3;
#[macro_export] macro_rules! φ {($($tokens:tt)*) => {if cfg!(debug_assertions){          pp!("{}",format!($($tokens)*))         } else{} }}
#[macro_export] macro_rules! φ1{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=1{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ2{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=2{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ3{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=3{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ4{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=4{pp!("{}",format!($($tokens)*))} else {}} else{} }}
#[macro_export] macro_rules! φ5{($($tokens:tt)*) => {if cfg!(debug_assertions){ if φL>=5{pp!("{}",format!($($tokens)*))} else {}} else{} }}
use φ as φ0;

pub fn ret42() -> i32 { 42 }
