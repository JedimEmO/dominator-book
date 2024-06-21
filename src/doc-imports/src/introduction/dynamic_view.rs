use dominator::{append_dom, body, clone, events, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use log::info;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
fn simple_mutable_signal() {
// ANCHOR: simple_mutable_signal
let x = Mutable::new(42_u32);
let x_signal_copied = x.signal();
let x_signal_cloned = x.signal_cloned();
// ANCHOR_END: simple_mutable_signal
}

#[rustfmt::skip]
fn simple_mutable_signal_ref() {
// ANCHOR: simple_mutable_signal_ref
    let x = Mutable::new(42_u32);
    let x_signal_ref = x.signal_ref(|new_value: &u32| {
        *new_value
    });
// ANCHOR_END: simple_mutable_signal_ref
}

// ANCHOR: simple_mutable_signal_for_each
async fn log_x(x_signal: impl Signal<Item = u32>) {
    x_signal
        .for_each(|v| {
            info!("Got new x: {}", v);
            async {}
        })
        .await;
}
// ANCHOR_END: simple_mutable_signal_for_each

#[rustfmt::skip]
async fn clone_example() {
// ANCHOR: clone
let value_a = Mutable::new(42);
let value_b = Mutable::new(666);

let my_lambda: &dyn Fn() -> () = &clone!(value_a, value_b => move || {
    value_a.set(1);
    value_b.set(1);
});

value_a.set(2);
// ANCHOR_END: clone
}
