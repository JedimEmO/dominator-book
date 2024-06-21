use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;

#[derive(Copy, Clone)]
pub enum Tutorial {
    DynamicView,
}

impl ToString for Tutorial {
    fn to_string(&self) -> String {
        match self {
            Tutorial::DynamicView => { "Dynamic View".to_string() }
        }
    }
}


pub struct TutorialStore {
    pub tutorials: MutableVec<Tutorial>,
    current_tutorial: Mutable<usize>,
}

impl TutorialStore {
    pub fn new() -> Self {
        Self {
            tutorials: MutableVec::new_with_values(vec![
                Tutorial::DynamicView
            ]),
            current_tutorial: Default::default(),
        }
    }

    pub fn current_tutorial_index_signal(&self) -> impl Signal<Item=usize> {
        self.current_tutorial.signal()
    }

    pub fn current_tutorial_signal(&self) -> impl Signal<Item=Tutorial> {
        let tutorials = self.tutorials.clone();

        self.current_tutorial.signal().map(move |tutorial_index| {
            tutorials.lock_ref()
                .iter()
                .enumerate()
                .find(|tut| tut.0 == tutorial_index)
                .map(|v| *v.1)
                .unwrap()
        })
    }
}