use doc_imports::design_essays::audio_slider::initial_design_final::audio_slider;
use dominator::{Dom, html};
use futures_signals::signal::{Mutable, SignalExt};

pub fn audio_slider_page() -> Dom {
    let state = Mutable::new(0f64);

    html!("div", {
        .children([
            html!("div", {
                .text_signal(state.signal().map(|v| format!("Slider value: {v}")))
            }),
            html!("br")
        ])
        .child(audio_slider(state.clone(), (10.0, 50.0)))
    })
}
