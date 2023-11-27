# The basics of FRP with futures-signals

Now that we have made a simple static html node, we'll pretty soon want to make it a bit more dynamic.
But before we dive into the code, we'll very briefly go over the fundamental principle of functional reactive programming (FRP).

The most important principle to understand is that in FRP, we consider the view to be a functional mapping of the state.
We typically refer to the result of such a mapping as a **derivation**.

Secondly, we consider the state to be a stream of values, not just a single value held in memory.

What does this mean?

Imagine that you have a variable `x` that holds the value `5`, and we want to turn it into the text `"5"`.

One way of doing this, of course, is to simply call `x.to_string()`.
This gives us the string representation of `x` at the time of the call.
This, however, is not very useful if we want to keep the text up to date with futures values of `x`.
If we reassign a new value to x, the string representation will remain the same old `"5"` as it was before.

Imagine now that instead of `x` holding the single value `5`, it is a stream of i32 values.
We can then map this stream to a stream of strings by calling `x.map(|x| x.to_string())`.
This gives us a new stream, which will yield the string representation of the latest value of `x` whenever `x` yields a new value.
Think of the stringified `x` as a **view** on the numerical value `x` holds at any given moment.

Values usually need to be stored however, so modelling them strictly as streams is not very feasible. 
`futures-signals` handles this by providing a collection of `Mutable` data containers.
They are `Mutable`, `MutableVec<T>` and `MutableBTreeMap<K,T>`.

What these have in common is that they store a value, and can give signals for the latest value held by the container.
Think of a signal as a regular async futures-streams `Stream`. 
They simply provide an **async** way of getting the next relevant value for a derivation.

In fact, there are utility methods provided to convert signals to and from regular Streams!

The specifics of how signals work vary slightly for the various types of signals.
For now, we will limit ourselves to `Mutable` for the introduction to the basic premises.
Don't worry, we will cover signals in more detail later, as they are crucial to understand in order to structure your application efficiently!

## Mutable<T>

This is the simplest of the mutable types.
It is a simple container, providing get/set methods for accessing the current held value directly.

> **Note:** If your type `T` is **Copy**, the `Mutable<T>` type will implement `.get()` and `.set()`. 
If `T` is **Clone**, there will be `.get_cloned()` and `.set_cloned()` instead

More importantly, `Mutable<T>` gives us a few ways to acquire a signal of the values it will hold.

The simplest signal we can get is when our type is `Copy`.
In this case, we can create a signal that copies the value forward like so:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/dynamic_view.rs:simple_mutable_signal}}
```

Now that we have a signal for all future values of `x`, we can write a function that should run on new values:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/dynamic_view.rs:simple_mutable_signal_for_each}}
```

One very important thing to be aware of regarding `Signal`, is that it may skip intermediate values when polled.
The delivery guarantee is that you will always poll the most recent value, but it may drop values if several updates happen in rapid succession.

This may sound strange, but it's important to mentally separate signals from streams.
When you chose to use signals, what you want to achieve is to perform a mapping of the latest state into a derivation.
You should not use `Signal` if what you wish to achieve is an element-by-element processing; this is what streams are for! 

## Let's make a dynamic view

Enough on signals; let's show a practical example.
Let's make a counter, where pressing a button will increment a value shown in a `<span>`.

If you recall from our static example, the `html!` macro allows us to set properties on the `Dom` node we are building by using the `.text()` call in the macro invocation.

DOMINATOR usually provides two (or sometimes more) such methods for any property we can set on the builder; one static and one dynamic version.
The dynamic counterpart normally has the suffix `_signal` or `_signal_vec` to communicate the type of signal it requires.

In our case, we know that we want to make a span with a text that changes according to a counter, so we use the `.text_signal()` and a mapping 

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/dynamic_view.rs:counter}}
```

There are a few key concepts to discuss here.

The first is to notice how we declare our component as a regular rust function.
We will discuss a few different approaches and patterns in the **Patterns** chapter.
For now, recognize that this function returns a single Dom node, which is allowed to mutate the `counter_value` argument.
One should be strict when declaring function arguments (in general, not just with dominator), so that the signature clearly describes the contract with the caller.
If we do not want to allow the function to mutate our value, we can either accept a `ReadOnlyMutable<u32>` or an `impl Signal<Item=u32>`.

### Events and lambdas

Another key concept it is important to get to terms with, is that lambdas we connect to JS events must be `'static`.
This means it can only capture static references, or by value.

What this practically means is that if we want to connect our events to the rest of our application in any meaningful way, we have two options:

- Create static references to the application state by `Box::leak()` and share `&'static` references to the relevant parts
- Keep state inside cloneable pointer types, and capture clones by values

Both of these are valid approaches, and typically a mix is good.
Again, the **Patterns** chapter will cover more of this.

For our current needs, it's good to be familiar with the `clone!` macro provided by DOMINATOR.
What it does is to take a list of comma separated values as the first argument, then after the fat arrow (`=>`) the code block we wish to move the clones into.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/dynamic_view.rs:clone}}
```

We here make a lambda that captures clones of the two values by value, using the `clone!` macro.

This is exactly what we do for the `.event()` callback we create in the counter component.
This lambda is connected to the JS click event, just as one would expect.
