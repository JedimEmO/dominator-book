use dominator::{Dom, html};

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

/*
// ANCHOR: first_main_dom
<h1>Hello,world!</h1>
// ANCHOR_END: first_main_dom
 */
}

#[rustfmt::skip]
fn children() -> Dom {
// ANCHOR: child
html!("div", {
    .child(html!("span", { .text("A child element") }))
    .children([
        html!("span", { .text("Another child") }),
        html!("span", { .text("Another child") }),
        html!("span", { .text("Another child") }),
    ])
})
// ANCHOR_END: child
}