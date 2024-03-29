use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::Arc;

// ANCHOR: on_click
pub fn my_shared_button(mut on_click: impl (FnMut() -> ()) + 'static) -> Dom {
    html!("button", {
        .event(move |_: events::Click| {
            on_click();
        })
    })
}
// ANCHOR_END: on_click

// ANCHOR: on_click_usage
pub fn use_onclick() -> Dom {
    let my_local_var = Mutable::new(0);

    html!("div", {
        .child(html!("span", { . text_signal(my_local_var.signal().map(|v| v.to_string()))}))
        .child(my_shared_button(clone!(my_local_var => move || {
            my_local_var.set(my_local_var.get() + 1)
        })))
    })
}
// ANCHOR_END: on_click_usage

#[rustfmt::skip]
mod oc_fn {
    use dominator::{clone, events, html, Dom};
    use futures_signals::signal::Mutable;
    use std::sync::Arc;
    use futures_signals::signal::SignalExt;
// ANCHOR: on_click_factory_fn
pub fn my_shared_button_factory(
    val: Mutable<i32>,
    on_click: impl (Fn() -> ()) + 'static,
) -> Dom {
    let on_click = Arc::new(on_click);

    html!("div", {
        .child_signal(val.signal().map(move |v| {
            Some(html!("button", {
                .event(clone!(on_click => move |_: events::Click| {
                    on_click();
                }))
            }))
        }))
    })
}
// ANCHOR_END: on_click_factory_fn
}

#[rustfmt::skip]
mod oc_fn_mut {
    use dominator::{html, Dom, clone, events};
    use futures_signals::signal::Mutable;
    use futures_signals::signal::SignalExt;
    
// ANCHOR: on_click_factory_fn_mut
pub fn my_shared_button_factory(
    val: Mutable<i32>,
    mut on_click: impl (FnMut() -> ()) + Clone + 'static,
) -> Dom {
    html!("div", {
        .child_signal(val.signal().map(clone!(on_click => move |v| {
            let mut on_click = on_click.clone();
            
            Some(html!("button", {
                .event(move |_: events::Click| {
                    on_click();
                })
            }))
        })))
    })
}
// ANCHOR_END: on_click_factory_fn_mut
}
#[rustfmt::skip]
mod oc_fn_mut_factory {
    use dominator::{clone, events, html, Dom};
    use futures_signals::signal::Mutable;
    use futures_signals::signal::SignalExt;

// ANCHOR: on_click_factory_fn_mut_factory
pub fn my_shared_button_factory<
    TFn: (FnMut() -> ()) + 'static,
    TFactory: (FnMut() -> TFn) + 'static,
>(
    val: Mutable<i32>,
    mut on_click_factory: TFactory,
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
// ANCHOR_END: on_click_factory_fn_mut_factory
}
