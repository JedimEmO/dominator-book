# Fragments

**DOMINATOR** provides a nice fragment system.
This is useful for providing a way of injecting children from outside the implementation of a component, without using a [mixin](mixins.md).

For instance, if we are making a list component, we need to let the user provide the content of our list:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:list}}
```

We can now use our list like this:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/fragments.rs:use_list}}
```

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