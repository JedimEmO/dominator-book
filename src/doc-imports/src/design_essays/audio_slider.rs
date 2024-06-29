#[rustfmt::skip]
pub mod initial_design_sketch {
    use dominator::{clone, Dom, events, html, svg, with_node};
    use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
    use web_sys::SvgElement;

// ANCHOR: audio_slider_sketch
pub fn audio_slider(value: Mutable<f64>, value_range: (f64, f64)) -> Dom {
    todo!()
}
// ANCHOR_END: audio_slider_sketch
}

#[rustfmt::skip]
pub mod initial_design_final {
    use dominator::{clone, Dom, events, html, svg, with_node};
    use futures_signals::signal::{Mutable, ReadOnlyMutable, SignalExt};
    use web_sys::SvgElement;
// ANCHOR: audio_slider_final
pub fn audio_slider(value: Mutable<f64>, value_range: (f64, f64)) -> Dom {
    let y_pos_signal = value.signal().map(move |v| {
        let value_scale = value_range.1 - value_range.0;
        let value_offset = value_range.0;

        let y_pos = 100.0 - 100.0 * (v - value_offset) / value_scale;

        y_pos.clamp(0.0, 100.0).to_string()
    });

    let button_state = Mutable::new(false);

    let calculate_value = move |element: &SvgElement, offset_y: i32| -> f64 {
        let height = element.get_bounding_client_rect().height();
        let value_scale = value_range.1 - value_range.0;
        let value_offset = value_range.0;

        (value_offset + value_scale * (1.0 - offset_y as f64 / height)).clamp(value_range.0, value_range.1)
    };

    html!("div", {
        .style("width", "40px")
        .style("height", "200px")
        .child(svg!("svg", {
            .with_node!(element => {
                .event(clone!(element, value, button_state => move |event: events::MouseMove| {
                    if button_state.get() {
                        value.set(calculate_value(&element, event.offset_y()));
                    }
                }))
                .child(svg!("rect", {
                    .attr("x", "6")
                    .attr("y", "5")
                    .attr("width", "6")
                    .attr("height", "100")
                    .attr("cursor", "pointer")
                    .event(clone!(element, button_state => move |event: events::MouseDown| {
                        button_state.set(true);
                        value.set(calculate_value(&element, event.offset_y()))
                    }))
                }))
            })
            .attr("viewBox", "0 0 20 110")
            .child(svg!("rect", {
                .apply(|builder| {
                    builder.global_event(clone!(button_state => move |event: events::MouseUp| {
                        button_state.set(false);
                    }))
                })
                .event(clone!(button_state => move |event: events::MouseDown| {
                    button_state.set(true);
                }))
                .attr_signal("y", y_pos_signal)
                .attr("width", "20")
                .attr("height", "10")
                .attr("fill", "gray")
                .attr("cursor", "pointer")
            }))
        }))
    })
}
// ANCHOR_END: audio_slider_final
}
