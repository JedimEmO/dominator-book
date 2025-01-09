// ANCHOR: test
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};

pub fn my_cmp() -> Dom {
    let counter = Mutable::new(0);

    html!("div", {
        .child(html!("button", {
            .attr("id", "click-me")
            .event(clone!(counter => move |_: events::Click| {
                counter.set(counter.get() + 1);
            }))
        }))
        .children_signal_vec(counter.signal_ref(|count| {
            (0..*count).map(|v| {
                html!("div", {
                    .class("count-me")
                    .text(v.to_string().as_str())
                })
            }).collect()
        }).to_signal_vec())
    })
}
#[cfg(test)]
mod in_browser_tests {
    use crate::techniques_and_patterns::in_browser_testing::my_cmp;
    use futures_signals::signal::{Mutable, SignalExt};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::HtmlElement;

    #[wasm_bindgen_test]
    async fn some_dom_test() {
        let cmp_to_test = my_cmp();

        // Make sure to replace the current body content, to avoid
        // multiple tests contaminating each other
        dominator::replace_dom(
            &dominator::body(),
            &dominator::body().first_child().unwrap(),
            cmp_to_test,
        );

        let button = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("click-me")
            .unwrap();

        let button_ref = button.dyn_ref::<HtmlElement>().unwrap();

        button_ref.click();
        button_ref.click();
        button_ref.click();

        // Yield execution time to the browser!
        // This is important; if omitted, the browser will not have time to update the DOM before we do our assertion!
        JsFuture::from(js_sys::Promise::resolve(&JsValue::null()))
            .await
            .unwrap();

        // Verify the resulting dom:
        let count_mes = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_elements_by_class_name("count-me");

        assert_eq!(count_mes.length(), 3);
    }
}
// ANCHOR_END: test
