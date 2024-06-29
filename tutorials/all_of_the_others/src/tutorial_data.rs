use std::fmt::Display;
use dominator::routing::{go_to_url, url};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tutorial {
    DynamicView,
    DesignEssays,
}

impl Tutorial {
    fn to_url(&self) -> String {
        match self {
            Self::DynamicView => "#/dynamic_view",
            Self::DesignEssays => "#/design_essays",
        }.to_string()
    }
}

impl Display for Tutorial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Tutorial::DynamicView => "Dynamic View",
            Tutorial::DesignEssays => "Design Essays",
        }
            .to_string();
        write!(f, "{}", str)
    }
}

pub struct TutorialStore {
    pub tutorials: MutableVec<Tutorial>,
}

impl TutorialStore {
    pub fn new() -> Self {
        Self {
            tutorials: MutableVec::new_with_values(vec![
                Tutorial::DynamicView,
                Tutorial::DesignEssays
            ])
        }
    }

    pub fn go_to_tutorial_index(&self, idx: usize) {
        let tutorial = self.tutorials.lock_ref()[idx];
        go_to_url(tutorial.to_url().as_str())
    }

    pub fn current_tutorial_index_signal(&self) -> impl Signal<Item = usize> {
        let tutorials = self.tutorials.clone();

        self.current_tutorial_signal().map(move |current_tutorial| {
            tutorials.lock_ref().iter().position(|v| v == &current_tutorial).unwrap()
        })
    }

    pub fn current_tutorial_signal(&self) -> impl Signal<Item = Tutorial> {
        url().signal_ref(move |new_url| {
            let url = web_sys::Url::new(new_url.as_str())
                .expect_throw("failed to construct url path");

            match url.hash().as_str() {
                "#/design_essays" => Tutorial::DesignEssays,
                _ => Tutorial::DynamicView
            }
        })
    }
}
