use dominator::{clone, fragment, html, with_node, Dom};
use futures_signals::signal::{always, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::UnwrapThrowExt;

// ANCHOR: with_node_example
fn with_node_example(blur_signal: impl Signal<Item = ()> + 'static) -> Dom {
    html!("div", {
        .with_node!(element => {
            .future(clone!(element => async move {
                blur_signal.to_future().await;
                // Call some functions on the real DOM node!
                element.blur().expect_throw("Failed to blur!");
                element.scroll_into_view()
            }))
            .apply(|builder| {
                info!("Inserting a {} node!", element.tag_name());
                builder
            })
        })
    })
}
// ANCHOR_END: with_node_example

#[rustfmt::skip]
fn multinode_example() -> Dom {
    let _ =
// ANCHOR: multinode
html!("div", {
    .child(html!("span", { .text("first child") }))
    .text("A text node")
    .child_signal(always(Some(html!("span", { .text("Some dynamic node") }))))
});
// ANCHOR_END: multinode
    /*
// ANCHOR: multinode_expanded
<div>
    <span>first child</span>
    A text node
    <span>Some dynamic node</span>
</div>
// ANCHOR_END: multinode_expanded
     */


// ANCHOR: multinode_multichild
let some_vec = MutableVec::new_with_values(vec![1, 2, 3 ]);
let some_other_vec = MutableVec::new_with_values(vec![1, 2, 3 ]);
let some_fragment = fragment!(move {
    .text("Hi there")
    .children_signal_vec(some_vec.signal_vec()
        .map(|v|
            html!("span", {
                .text(format!("Dynamic child in fragment #{v}").as_str())
            })))
});

html!("div", {
    .fragment(&some_fragment)
    .children_signal_vec(some_other_vec.signal_vec()
        .map(|v|
            html!("span", {
                .text(format!("Dynamic child fragment #{v}").as_str())
            })))
})
// ANCHOR_END: multinode_multichild
}
