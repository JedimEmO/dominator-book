use dominator::{html, Dom};
use futures_signals::signal::{Mutable, ReadOnlyMutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};

#[rustfmt::skip]
pub fn mut_vec() -> Dom {
// ANCHOR: mut_vec_dumb
let data = Mutable::new(vec![1,2,3]);
let dom_vec_signal = data.signal_ref(|values: &Vec<i32>| {
    values.iter().map(|value: &i32| {
        html!("div", {
            .text(&format!("{value}"))
        })
    }).collect::<Vec<_>>()
});

let mut changed_vec = data.get_cloned();

changed_vec.pop();
changed_vec[0] = 666;
data.set(changed_vec);
// ANCHOR_END: mut_vec_dumb

let _ = html!("div", {
    .children_signal_vec(dom_vec_signal.to_signal_vec())
});
    // ANCHOR: mut_vec
let data = MutableVec::new_with_values(vec![1,2,3]);
let dom_signal_vec = data.signal_vec().map(|value: i32| {
    html!("div", {
        .text(&format!("{value}"))
    })
});

data.lock_mut().pop();
data.lock_mut().set(0, 42);

html!("div", {
    .children_signal_vec(dom_signal_vec)
})
// ANCHOR_END: mut_vec

}

#[rustfmt::skip]
fn flatten_example() -> Dom {
// ANCHOR: flattened_mut_vec
let items = MutableVec::new_with_values(vec![1, 2, 3]);
let nested_signal_vec = items
    .signal_vec()
    .map(|item: i32| {

    futures_signals::signal_vec::always(
        (0..item).map(|value| {
            format!("{item}:{value}")
        })
        .collect()
    )
});

let flattened_dom_signal_vec = nested_signal_vec.flatten().map(|v: String|{
    html!("span", { .text(&v)})
});

items.lock_mut().set(1, 5);

html!("div", {
    .children_signal_vec(flattened_dom_signal_vec)
})
// ANCHOR_END: flattened_mut_vec
}

// ANCHOR: signal_conversion
fn consume_signal_vec(sig: impl SignalVec<Item = ()>) {}
fn consume_signal_cloned(sig: impl Signal<Item = Vec<()>>) {}

fn conversion() {
    let v1 = Mutable::new(vec![()]);
    let v2 = MutableVec::new_with_values(vec![()]);

    // Call a function expecting SignalVec<Item=()>
    consume_signal_vec(v1.signal_cloned().to_signal_vec());

    // Call a function expecting Signal<Item=Vec<()>>
    consume_signal_cloned(v2.signal_vec().to_signal_cloned());
}
// ANCHOR_END: signal_conversion

#[rustfmt::skip]
fn enumerate() {
// ANCHOR: enumerate
let data = MutableVec::new_with_values(vec![1, 2, 3]);
data.signal_vec()
    .enumerate()
    .map(|(index, value): (ReadOnlyMutable<Option<usize>>, i32)| {
        html!("div", {
            .text_signal(index.signal_ref(|idx| {
                match idx {
                    Some(idx) => format!("I am at index {idx}"),
                    _ => format!("I am removed!")
                }
            }))
        })
});
// ANCHOR_END: enumerate
}
