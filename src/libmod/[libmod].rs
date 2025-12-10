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

#[macro_export] macro_rules! φ  {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
            println!("{}",format!( $($tokens)*));     #[cfg(not(debug_assertions))]{}    }}
#[macro_export] macro_rules! φ1 {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
  if φL>=1{println!("{}",format!( $($tokens)*))}else{}#[cfg(not(debug_assertions))]{}    }}
#[macro_export] macro_rules! φ2 {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
  if φL>=2{println!("{}",format!( $($tokens)*))}else{}#[cfg(not(debug_assertions))]{}    }}
#[macro_export] macro_rules! φ3 {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
  if φL>=3{println!("{}",format!( $($tokens)*))}else{}#[cfg(not(debug_assertions))]{}    }}
#[macro_export] macro_rules! φ4 {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
  if φL>=4{println!("{}",format!( $($tokens)*))}else{}#[cfg(not(debug_assertions))]{}    }}
#[macro_export] macro_rules! φ5 {($($tokens:tt)*) => {#[cfg(    debug_assertions)]
  if L5>=4{println!("{}",format!( $($tokens)*))}else{}#[cfg(not(debug_assertions))]{}    }}
use φ as φ0;

pub fn ret42() -> i32 { 42 }
