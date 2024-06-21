use dominator::{clone, Dom, events, html};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;

pub fn dynamic_view() -> Dom {
    let counter_data = Mutable::new(0);
    counter(counter_data)
}

// ANCHOR: counter
pub fn counter(counter_value: Mutable<u32>) -> Dom {
    let counter_text_signal = counter_value
        .signal()
        .map(|new_value| format!("The counter value is {}", new_value));

    html!("div", {
        .child(html!("h1", {
            .text_signal(counter_text_signal)
        }))
        .child(html!("button", {
            .text("Increase!")
            .event(clone!(counter_value => move |_: events::Click| {
                counter_value.set(counter_value.get() + 1);
            }))
        }))
    })
}
// ANCHOR_END: counter
