# Working with callbacks

When making shared components, we usually want to customize certain behaviours of the component.
The common way to solve this, is with callbacks.

With dominator, we can easily do this with regular `'static` rust closures:

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/working_with_callbacks.rs:on_click}}
```

Here's an example of using this:

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