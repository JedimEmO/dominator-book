#[macro_use]
extern crate dmat_components;

mod tutorial_data;
mod tutorials;

use crate::tutorial_data::Tutorial;
use crate::tutorials::design_essays::design_essays;
use crate::tutorials::dynamic_view::dynamic_view;
use dmat_components::components::app_bar::*;
use dmat_components::components::tabs::*;
use dominator::{append_dom, body, html, Dom};
use futures_signals::signal::SignalExt;
use tutorial_data::TutorialStore;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main_js() {
    let _logger = wasm_logger::init(Default::default());
    let tutorial_store = Box::leak(Box::new(TutorialStore::new()));

    append_dom(&body(), tutorial(tutorial_store));
}

pub fn tutorial(tutorial_store: &'static TutorialStore) -> Dom {
    app_bar!({
        .apply(|b| b.attr("id", "tutorial-app"))
        .header(Some(tutorial_header(tutorial_store)))
        .main_signal(tutorial_store.current_tutorial_signal().map(|current_tutorial| tutorial_body(current_tutorial)).map(Some))
        .fixed(true)
    })
}

fn tutorial_body(tutorial: Tutorial) -> Dom {
    match tutorial {
        Tutorial::DynamicView => dynamic_view(),
        Tutorial::DesignEssays => design_essays(),
    }
}

fn tutorial_header(tutorial_store: &'static TutorialStore) -> Dom {
    html!("div", {
        .children(vec![
            html!("h1", { .text("DOMINATOR Tutorials")}),
            tutorial_tabs(tutorial_store)
        ])
    })
}

fn tutorial_tabs(tutorial_store: &'static TutorialStore) -> Dom {
    tabs!({
        .active_tab_signal(tutorial_store.current_tutorial_index_signal().map(Some))
        .tab_click_handler(|idx| {
            tutorial_store.go_to_tutorial_index(idx)
        })
        .tabs(tutorial_store.tutorials.lock_ref().iter().map(|tutorial| {
            html!("span", { .text(tutorial.to_string().as_str()) })
        }).collect::<Vec<_>>())
    })
}
