# Working with callbacks

When making shared components, we usually want to customize certain behaviours of the component.
The common way to solve this, is with callbacks.

With dominator, we can easily do this with regular `'static` rust closures:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click}}
```

It's important to be familiar with the `clone!` macro provided by DOMINATOR.
What it does is to take a list of comma separated values as the first argument, then after the fat arrow (`=>`) the code block we wish to move the clones into.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/dynamic_view.rs:clone}}
```

We here make a lambda that captures clones of the two values by value, using the `clone!` macro.

Here's an example of using our button:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click_usage}}
```

Note that any callback handed over to the JS runtime must have a `'static` lifetime.
This means it must capture everything that isn't a `'static` lifetime reference by value!

## Reusing closures 

Sometimes, we need to provide a callback function that will be handed over to multiple callers.
There are a few ways to do this, depending on what type of closure you wish to use.

### `Fn` closure
The simplest is to use an `Fn` closure, which we can wrap in an `Arc` internally, and hand over to our event handlers:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click_factory_fn}}
```

### Cloneable closure

If we need to use an `FnMut` closure, we can constrain the closure to be `Clone`.
This necessary, since the `Arc` trick above will not allow us to access the closure as mutable within the `Arc`.

A closure implements `Clone` as long as all of its captures variables also are `Clone`.
Indeed, closures are also `Copy` as long as all captured members are copy.

Since we know the closure is `Clone`, we can easily clone and move into the event handlers as needed:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click_factory_fn_mut}}
```

### Closure factory

And finally, if for some reason our `FnMut` closure cannot be `Clone`, we can adopt a factory pattern.
This is simply a wrapping lambda, which returns a new closure for each invocation:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click_factory_fn_mut_factory}}
```


### Dealing with &'static

What this practically means is that if we want to connect our events to the rest of our application in any meaningful way, we have two options:

- Create static references to the application state by `Box::leak()` and share `&'static` references to the relevant parts
- Keep state inside cloneable pointer types, and capture clones by values

Both of these are valid approaches, and typically a mix is good.
Again, the **Patterns** chapter will cover more of this.
