# Mixins

*Mixins* is a pattern that can be used to allow the user access to its own builder.
For instance, we may want to give the user a way of setting the ID of our component, or to attach some custom event handler to it.

Here's a very basic example.
The `component_with_mixin` component allows its user to attach to its top level elements' `apply` function:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:component_with_mixin}}
```

The user can now customize essentially anything it wants on the component.
Here's an example setting a CSS class:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:using_component_with_mixin}}
```

<div class="warning">
By allowing access to your components builder, it's possible for the user to create conflicts with the invariants specified for your components.
</div>

## Making generic mixin implementations

Allowing mixins to be applied to components that we write is good, but we can also implement generic functionality  as mixin factories.

Let's say we wish to get a signal for the hovered state of any element.
We could of course copy and paste the code around, but it seems like a bad practice.

We can instead create a higher order function, which produces a mixin that we can pass to any component that we wish to support this hover functionality for!

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:hover_signal_mixin}}
```

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:using_hover_signal_mixin}}
```
