#[macro_use]
extern crate dwui;

#[macro_use]
extern crate dwind_macros;

mod tutorial_data;
mod tutorials;

use crate::tutorial_data::Tutorial;
use crate::tutorials::design_essays::design_essays;
use crate::tutorials::dynamic_view::dynamic_view;
use dominator::{body, clone, events, html, replace_dom, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use tutorial_data::TutorialStore;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main_js() {
    dwind::stylesheet();
    let _logger = wasm_logger::init(Default::default());
    let tutorial_store = Box::leak(Box::new(TutorialStore::new()));
    replace_dom(
        &body().parent_node().unwrap(),
        &body(),
        tutorial(tutorial_store),
    );
}

pub fn tutorial(tutorial_store: &'static TutorialStore) -> Dom {
    html!("body", {
        .dwclass!("text-woodsmoke-50 bg-woodsmoke-900")
        .dwclass!("flex justify-center")

        .child(html!("div", {
            .dwclass!("w-lg")
            .dwclass!("grid bg-woodsmoke-800 rounded")
            .style("grid-template-rows", "60px 1fr")
            .child(tutorial_header(tutorial_store))
            .child(html!("div", {
                .dwclass!("p-l-4 p-r-4")
                .child_signal(
                    tutorial_store
                        .current_tutorial_signal()
                        .map(|current_tutorial| tutorial_body(current_tutorial)).map(Some))
            }))
        }))
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
        .dwclass!("flex flex-row justify-center align-items-center gap-10")
        .dwclass!("border-b border-woodsmoke-700 divide-x")
        .children(vec![
            html!("h1", {
                .dwclass!("text-xl font-extrabold")
                .text("DOMINATOR Tutorials")
            }),
            tutorial_tabs(tutorial_store)
        ])
    })
}

fn tutorial_tabs(tutorial_store: &'static TutorialStore) -> Dom {
    let tutorials = tutorial_store
        .tutorials
        .lock_ref()
        .iter()
        .enumerate()
        .map(|(idx, tutorial)| {
            html!("span", {
                .dwclass!("m-x-2 transition-all")
                .dwclass!("cursor-pointer border-woodsmoke-500 hover:border-b")
                .dwclass_signal!(
                    "border-b border-woodsmoke-50 hover:border-woodsmoke-500",
                    tutorial_store
                        .current_tutorial_index_signal()
                        .map(move |current_tutorial| current_tutorial == idx)
                )
                .event(clone!(tutorial_store => move |_: events::Click| {
                    tutorial_store.go_to_tutorial_index(idx)
                }))
                .text(tutorial.to_string().as_str())
            })
        })
        .collect::<Vec<_>>();

    html!("div", {
        .dwclass!("flex flex-row align-items-center")
        .children(tutorials)
    })
}
