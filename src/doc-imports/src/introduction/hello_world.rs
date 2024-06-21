#[rustfmt::skip]
#[cfg(not(target_arch = "wasm32"))]
mod hidden_start {
// ANCHOR: first_main
use dominator::{append_dom, body, html};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    append_dom(&body(), html!("h1", {
   .text("Hello, world!")
}));
}
// ANCHOR_END: first_main
}