# Mixins

*Mixins* is a pattern that can be used to allow the user access to a components builder.
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

## Avoiding footgunnyness

As the warning above states, it can be risky to provide full access to your internal builder.
This can allow the user to break invariants assumed to be in place for your internal implementation, so use mixins a bit carefully.

### Use fragments

If what we wish to do is to let the user inject one or more children into a section of our components DOM tree, we may wish to use [fragments](fragments.md) instead.

### More precise arguments
It's a good idea to consider more targeted callback functions and signals.
If you want the user to be allowed to provide a class signal, then simply accept the class signal:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:class_signal}}
```

This lets the consumer do a more controlled customization of your component, and doesn't risk the issues associated with mixins.


### Builder wrapper

Alternatively, we can wrap the builder in a struct to expose only certain operations:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/mixins.rs:builder_wrapper}}
```

