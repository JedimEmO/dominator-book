use doc_imports::design_essays::audio_slider::initial_design_final::audio_slider;
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};

pub fn audio_slider_page() -> Dom {
    let state = Mutable::new(0f64);
    let disabled = Mutable::new(false);

    html!("div", {
        .children([
            html!("label", {
                .text("Disabled")
                .attr("for", "disabled")
            }),
            html!("input", {
                .attr("type", "checkbox")
                .attr("name", "disabled")
                .event(clone!(disabled => move |change: events::Change| {
                    disabled.set(change.checked().unwrap())
                }))
            }),
            html!("div", {
                .text_signal(state.signal().map(|v| format!("Slider value: {v}")))
            }),
            html!("br")
        ])
        .child(audio_slider(state.clone(), (10.0, 50.0), disabled.read_only()))
    })
}
