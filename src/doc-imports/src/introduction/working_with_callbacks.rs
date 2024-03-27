use dominator::{events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};

// ANCHOR: on_click
pub fn my_shared_button(mut on_click: impl (FnMut() -> ()) + 'static) -> Dom {
    html!("button", {
        .event(move |_: events::Click| {
            on_click();
        })
    })
}
// ANCHOR_END: on_click

// ANCHOR: on_click_factory
pub fn my_shared_button_factory<
    TCallback: (FnMut() -> ()) + 'static,
    TCallbackFactory: (FnMut() -> TCallback) + 'static,
>(
    val: Mutable<i32>,
    mut on_click_factory: TCallbackFactory,
) -> Dom {
    html!("div", {
        .child_signal(val.signal().map(move |v| {
            let mut on_click = on_click_factory();
            Some(html!("button", {
                .event(move |_: events::Click| {
                    on_click();
                })
            }))
        }))
    })
}
// ANCHOR_END: // ANCHOR: on_click_factory
