# Mutable vector

`Mutable<T>` is awesome if you are storing single values.
But if you have a `Mutable<Vec<T>>`, we must always update the entire vec when we wish to change it.
Consider this example.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_collections.rs:mut_vec_dumb}}
```

When an element is added to, removed from or changed in the vector, we have to redraw all the elements in our list.
This is perhaps ok for very small vectors, but it is a big performance hit when dealing with bigger lists,
since it will cause massive unnecessary layout recalculations.

But we can do better.
In fact, we can achieve near perfect granularity when updating the dom by using `MutableVec<T>` and the associated
`SignalVec<Item=T>`.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_collections.rs:mut_vec}}
```

As you see in the latter example, we can change individual elements of a vec.
This propagates through the `SignalVec` we get from the `MutableVec` instance by simply forwarding the diff produced by
the mutation.

Notice how the `DomBuilder` has a `children_signal_vec()` method.
This accepts a `SignalVec<Dom>`, and will ensure optimal DOM updates based on the changes to the signal.

It's the signal counterpart to the `.children()` method we saw in the introductory chapter!

This means we can map the `SignalVec` on a per-element basis, and dominator in turn can optimally update only the parts
of the DOM that require changing as a result of a diff!
This ensures that we don't cause any needless layout shifts or recalculations, which is essential for writing performant
web applications.

## SignalVec in SignalVec

Sometimes, we run into the "sibling problem" when composing UIs.
This typical if we have a component that wishes to have a sibling dependent on some internal state.
For instance, if we are rendering a table view, and a row wishes to have a following containing more details if it is "
expanded".

We can solve this by nesting `SignalVec` inside of `SignalVec`, and then use the `.flatten()` method to convert into a
single dimension `SignalVec`.

The following example shows how we convert a `SignalVec<Item=i32>` to a `SignalVec<Item=SignalVec<Item=String>>`, where
the inner
signal
vec has a number of rows equal to its input value.
We then use `.flatten()` to convert it into a `SignalVec<Item=String>` before we finally map it to a
`SignalVec<Item=Dom>`.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_collections.rs:flattened_mut_vec}}
```

Pay attention to what happens when we replace the element at index 1 with the number 5.
The signal vec at position 1 will receive an update, re-evaluate and produce an output containing 5 instead of 3 rows.

## Signal<Vec<<T>> -> SignalVec<T> -> Signal<Vec<T>>

Sometimes, we want to fit a square peg into a round hole.
We may wish to convert between `Signal<Item=Vec<T>>` and `SignalVec<Item=T>` for reasons outside our control.

Luckily, this is trivially done using the `.to_signal_vec()` and `to_signal_cloned()` methods.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_collections.rs:signal_conversion}}
```

Note that if you have a `SignalVec` made this way, any update in the root signal will cause the entire vec to be
replaced;
this is simply a type coercion and not a magical incantation.

If you need fine-grained updates, make sure you convert your data model into using a `MutableVec` properly!

## Enumerate

We can call `.enumerate()` on a signal vec to convert it into a `SignalVec<Item=(ReadOnlyMutalbe<Option<usize>>, T)>`.
The read only mutable contains the index of each element, so this behaves similar to the standard iterator
`enumerate()` function.

The reason the index is a mutable rather than just a number is that when reordering or removing elements in a vector,
we can get a signal to the changed index (or lack thereof) for each element.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_collections.rs:enumerate}}
```