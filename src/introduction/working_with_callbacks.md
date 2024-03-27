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

Sometimes, we need to provide a callback function that will be handed over to multiple callers.
One way to do this, is to provide a factory closure:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click_factory}}
```


### Dealing with &'static

What this practically means is that if we want to connect our events to the rest of our application in any meaningful way, we have two options:

- Create static references to the application state by `Box::leak()` and share `&'static` references to the relevant parts
- Keep state inside cloneable pointer types, and capture clones by values

Both of these are valid approaches, and typically a mix is good.
Again, the **Patterns** chapter will cover more of this.
