use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use futures_signals::signal_map::{MutableBTreeMap, SignalMapExt};
use futures_signals::signal_vec::SignalVecExt;
use web_sys::HtmlElement;

#[rustfmt::skip]
fn mutable_map_example_1() -> Dom {
// ANCHOR: mutable_map_1
let mutable_map: MutableBTreeMap<String, String> = MutableBTreeMap::new();

// Gives us a signal vec of all the keys in the map
let keys_signal = mutable_map.signal_vec_keys().map(|key| {
    html!("div", {
        .text(&key)
    })
});

// Gives a signal vec of all the (key, value) entries in the map
let entries_list_signal = mutable_map.entries_cloned().map(|(key, value)| {
    html!("dom", {
        .text(&format!("{key}: {value}"))
    })
});

html!("div", {
    .children_signal_vec(keys_signal)
    .children_signal_vec(entries_list_signal)
})
// ANCHOR_END: mutable_map_1
}

#[rustfmt::skip]
fn mutable_map_example_key_cloned() -> Dom {
// ANCHOR: mutable_map_key_cloned
let mutable_map: MutableBTreeMap<&str, String> = MutableBTreeMap::new();

// Get a signal for the optional value for a given key in the map
let key_signal = mutable_map
    .signal_map_cloned()
    .key_cloned("my-key")
    .map(|value: Option<String>| {
        match value {
            Some(v) => format!("my-key has value: {v}"),
            _ => "my-key has no value in the map".to_string()
        }
    }
);

html!("div", {
    .text_signal(key_signal)
})
// ANCHOR_END: mutable_map_key_cloned
}
