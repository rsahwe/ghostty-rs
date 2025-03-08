#![crate_name = "ghostty"]
#![crate_type = "lib"]
#![allow(non_upper_case_globals)]

pub extern crate ghostty_sys as sys;

#[cfg(test)]
pub mod tests;

pub mod ghostty;
