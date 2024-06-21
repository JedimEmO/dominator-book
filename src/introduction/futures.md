# Async and futures

It's fair to say that when writing a web application, you are going to have to deal with async programming.
A lot.

There's a good chance you have to make async calls to your backend.
Most likely your users will trigger actions you wish to perform over time without blocking the single threaded runtime of the browser.
You may wish to have timer based events happening in your UI.

Any of these requires some form of `async` support in your code.

Luckily, rust has a very good async model, and DOMINATOR lets us use it from our components easily!

## Futures and components

First of all, let's assume we have an interface to our backend with an async method providing us some item information:

```rust
{{#include ../doc-imports/src/introduction/futures.rs:trait}}
```

This provides a nice abstraction over the asynchronous HTTP calls required to retrieve the information from our backend.
Now, let's say we wish to call this method when we are rendering our view, so that we can properly display the information when it becomes available.

A good strategy for this is to create a `Mutable` for holding our value, initialized to `None`.
We then make the request, and whenever the response returns we will populate the mutable with the received value.

This lets us render the view as a simple signal mapping over the optional value:

```rust
{{#include ../doc-imports/src/introduction/futures.rs:renderer}}
```

But we need to drive the future returned by our function somehow!

Luckily, dominator lets us associate futures with our elements, and will make sure they are polled.

```rust
{{#include ../doc-imports/src/introduction/futures.rs:component}}
```

As we can see, this lets us write straight forward async rust code, and allows us to tie it back to our state by capturing the relevant state into our futures.

Be aware that when the element is dropped, the futures associated with it will no longer be polled.

## Spawning tasks with spawn_local

Sometimes we have to spawn tasks that doesn't exist at the construction of our elements.

For this, we will have to use the `wasm-bindgen-futures` crate, which provides a handy `spawn_local` function!

```rust
{{#include ../doc-imports/src/introduction/futures.rs:spawn_local}}
```
