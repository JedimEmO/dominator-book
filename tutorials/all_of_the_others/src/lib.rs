#[macro_use]
extern crate dmat_components;

mod tutorials;
mod tutorial_data;

use dominator::{append_dom, body, Dom, html};
use dmat_components::components::app_bar::*;
use dmat_components::components::tabs::*;
use futures_signals::signal::SignalExt;
use wasm_bindgen::prelude::*;
use tutorial_data::{TutorialStore};
use crate::tutorial_data::Tutorial;
use crate::tutorials::dynamic_view::dynamic_view;

#[wasm_bindgen(start)]
fn main_js() {
    let tutorial_store = Box::leak(Box::new(TutorialStore::new()));

    append_dom(
        &body(),
        tutorial(tutorial_store),
    );
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
    match tutorial { Tutorial::DynamicView => dynamic_view() }
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
        .tabs(tutorial_store.tutorials.lock_ref().iter().map(|tutorial| {
            html!("span", { .text(tutorial.to_string().as_str()) })
        }).collect::<Vec<_>>())
    })
}