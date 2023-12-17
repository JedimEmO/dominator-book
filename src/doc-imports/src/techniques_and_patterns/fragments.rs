use dominator::{append_dom, body, box_fragment, fragment, html, BoxFragment, Dom, Fragment};
use futures_signals::signal::SignalExt;
use futures_signals::signal::{always, Mutable};
use futures_signals::signal_vec::SignalVecExt;

// ANCHOR: list
fn list(children: impl Fragment) -> Dom {
    html!("ul", {
        .fragment(&children)
    })
}
// ANCHOR_END: list

// ANCHOR: use_list
fn use_my_list() -> Dom {
    list(fragment!({
        .text("Hello, world!")
        .child(html!("span", { .text("A static child ")}))
        .child_signal(always(true).map(|_| Some(html!("span", { .text("Dynamic child")}))))
        .children([ html!("span", {.text("List of children")})])
        .children_signal_vec(futures_signals::signal_vec::always(vec![0,1]).map(|idx| html!("span", { .text(format!("Dynamic children #{}", idx).as_str())})))
    }))
}

// ANCHOR_END: use_list

// ANCHOR: redraw_with_children
fn redraw_with_children(children: impl Fragment + 'static) -> Dom {
    html!("ul", {
        .child_signal(always(true).map(move |_| Some(html!("li", {
            .fragment(&children)
        }))))
    })
}
// ANCHOR_END: redraw_with_children

// ANCHOR: boxed_fragment
struct MyComponent {
    fragments: BoxFragment,
}

impl MyComponent {
    pub fn render(&self) -> Dom {
        html!("div", {
            .fragment(&self.fragments)
        })
    }
}
// ANCHOR_END: boxed_fragment

#[rustfmt::skip]
fn use_my_component() {
// ANCHOR: use_boxed_fragment
let my_cmp = MyComponent {
    fragments: box_fragment!({ .text("hi there!")}),
};

append_dom(&body(), my_cmp.render());

// ANCHOR_END: use_boxed_fragment
}

// ANCHOR: move_fragment
fn move_fragment() -> impl Fragment {
    let value = Mutable::new(42);
    fragment!(move { .text_signal(value.signal().map(|v| format!("{}", v)))})
}
// ANCHOR_END: move_fragment
