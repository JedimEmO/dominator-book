# Routing

Any modern web application needs to support routing.
**DOMINATOR** provides some utilities on top of the browsers history APIs to make integrating routing with our futures-signals based application state easier.
Let's get started!

## url()

The starting point is `dominator::routing::url()`, which returns a `ReadOnlyMutable<String>`. 
This allows us to read the current url directly, but even more importantly, it lets us get a signal for it!

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:route_url}}
```

To change the current URL, we use the `goto_url` function

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:change_url}}
```

We can make a simple application with two routes: `#/hello` and a default.

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:hello_world_url}}
```

Note how we create a `web_sys::Url` from the route string.
This gives some utility, for our case we mostly care about the `hash()` method to make matching on relative URLs easier.

## The route enum 

The above example may be all we need for a simple application, but there are also cases were we wish to avoid directly hard coding route strings all over our application code.
There are several ways of solving this, one of which is to model the route states as rust enums.
We can then write perfectly type-safe routing code, and it also gives us a very structured model of the possible application states!

Let's create an imaginary application.
It has a landing page, which is also the default route.
There's a "Shop now" view, which additionally can have an item ID as a sub-view for direct linking to items.

We can start by declaring the route enum:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:route_enum}}
```

And our application can accept an instance of this enum to render:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:route_enum_render}}
```

With that out of the way, we need to somehow connect to the routing itself.
Again, there are several ways to do this, but one compact and nice-ish way is to create a few static methods on the enum itself.

We need a function to provide us with a signal of the enum value representing the current route.
We also need to be able to modify the route.

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:route_enum_methods}}
```

Now we can put it all together to render a strongly typed, routed application:

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:route_enum_full}}
```

Notice we use the `link!` macro provided by tokio for a shorthand.
This creates an `<a>` element with on click set correctly for us!
 
## Generalized router

We can generalize and simplify the route handling a bit.
For instance, we can build a generic router based on the `matchit` crate (which is use din the popular backend library `axum`).

It is essentially a set of routes, associated with a lambda function to extract the matched parameters and translate them into our desired route value (typically our enum from up-top)

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:generalized_router}}
```

This makes it super easy to wire our application routes using lambdas.
We simply register handler functions for our chosen route patterns on the `matchit` router instance.

These lambdas then return the value we wish to receive from the app routers signal.

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/routing.rs:matchit_routing}}
```