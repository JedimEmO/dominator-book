#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate log;

pub mod design_essays;
pub mod introduction;
pub mod techniques_and_patterns;

#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test_configure;
#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);
