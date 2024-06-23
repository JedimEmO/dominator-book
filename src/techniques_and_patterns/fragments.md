# Fragments

There are many situations where you wish to expose all or parts of your child list to the user of a components, but you may not want to give them full freedom of selecting all the children.
It may also not always be feasible to require a set of children to conform to a certain trait, such as `impl SignalVec` etc. to expose only parts of your child list to the caller.

This is what **DOMINATOR** `Fragment`s are for.
They are useful for providing a way of injecting children from outside the implementation of a component, without using a [mixin](mixins.md).

On a technical note, the `Fragment` trait exposes an apply method, which will apply the fragments method block onto the `DomBuilder` for the element the fragment is inserted into.
There is also a handy macro, `fragment!`, which allows us to declare fragments essentially how we would use the `html!` macro, but without the tag name.

For instance, if we are making a list component, we need to let the user provide the content of our list:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:list}}
```

We can now use our list like this:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:use_list}}
```

Observe how we are free to implement our fragment in any way we wish, i.e. use both static, dynamic or a combination of these children.
However, the implementation of the component gets to chose where and when we insert that group of children into its DOM tree!

Since fragments are essentially implemented as a factory closure, we can even apply them several times.
This is handy if we need to redraw the containing element for some reason!

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:redraw_with_children}}
```

## BoxFragment

If you need to store the fragment in a struct, we have to use `BoxFragment` and `box_fragment!` instead:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:boxed_fragment}}
```

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:use_boxed_fragment}}
```

## Moving into fragments

If you are returning a fragment from a function, you will have to capture any state it references with `move`:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:move_fragment}}
```

The returned fragment will here own the value in its closure, and is as such free to be passed around as a value.