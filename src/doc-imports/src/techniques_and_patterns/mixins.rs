use dominator::events::{MouseEnter, MouseLeave};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{Mutable, MutableSignal, Signal, SignalExt};
use web_sys::HtmlElement;

// ANCHOR: component_with_mixin
fn component_with_mixin(
    mixin: impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
) -> Dom {
    html!("div", {
        .apply(mixin)
        .text("I support a mixin")
    })
}
// ANCHOR_END: component_with_mixin

// ANCHOR: using_component_with_mixin
fn using_a_mixin() -> Dom {
    html!("div", {
        .child(component_with_mixin(|builder| builder.class("added-from-user")))
        .text("I applied a mixin to my child!")
    })
}
// ANCHOR_END: using_component_with_mixin

// ANCHOR: hover_signal_mixin
#[derive(Copy, Clone, Debug)]
enum HoverState {
    Hovered,
    NotHovered,
}

fn hover_signal_mixin(
    callback: impl FnOnce(DomBuilder<HtmlElement>, MutableSignal<HoverState>) -> DomBuilder<HtmlElement>,
) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    let is_hovered = Mutable::new(HoverState::NotHovered);

    move |builder| {
        builder
            .apply(clone!(is_hovered =>  move |builder|
                callback(builder, is_hovered.signal())))
            .event(clone!(is_hovered => move |_: MouseEnter|
                is_hovered.set(HoverState::Hovered)))
            .event(clone!(is_hovered => move |_: MouseLeave|
                is_hovered.set(HoverState::NotHovered)
            ))
    }
}
// ANCHOR_END: hover_signal_mixin

// ANCHOR: using_hover_signal_mixin
fn using_hover_signal_mixin() -> Dom {
    component_with_mixin(hover_signal_mixin(|builder, hover_signal| {
        builder.class_signal(
            "hovered",
            hover_signal.map(|hover_state| match hover_state {
                HoverState::Hovered => true,
                _ => false,
            }),
        )
    }))
}
// ANCHOR_END: using_hover_signal_mixin

// ANCHOR: class_signal
fn component_with_class_signal(active_class_signal: impl Signal<Item = bool> + 'static) -> Dom {
    html!("div", {
        .class_signal("active", active_class_signal)
    })
}
// ANCHOR_END: class_signal
