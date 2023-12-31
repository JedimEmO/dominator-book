# Logging and Debugging

Debugging a wasm UI can be slightly more complex than a JS based web application.
This is primarily due to the lack of source map support in our tooling.

However, it's not as difficult as many make it out to be.
The browser supports DWARF debugging symbols, which are generated by the rust compiler.

## Enable debug builds

When using rollup, we need to enable debug builds.
This is done via a parameter to the rust plugin, configured in the rollup configuration file.

```js
plugins: [
    rust({
        serverPath: "js/",
        debug: true
    })
]
```

When debug builds are enabled, we get a much more meaningful error message from the browser:

![Panic message with debug symbols](images/panic_with_debug.png)

As opposed to the (very) unhelpful release build message:

![Panic message without debug symbols](images/panic_no_debug.png)


## Use unwrap_throw

Another good practice is to use `unwrap_throw()` and `expect_throw()` as opposed to `unwrap()` and `expect()`.
The `_throw()` counterpart will create a nicer JS exception than the generic runtime error caused by a panic.

This improved exception contains source information that greatly improves the debugging experience:

![Panic message without debug symbols](images/panic_unwrap_throw.png)

## Logging

The `log` crate provides a fairly ubiquitous interface for various logging tasks in the rust ecosystem.
The `wasm-logger` crate gives us the option to use this from the browser as well (targeting the console, as one might expect).

Simply add the `log` and `wasm-logger` crates as dependencies, and initialize logging from your main function:

```rust,no_run,noplayground
wasm_logger::init(wasm_logger::Config::default());
```

You can now use the various logging macros from `log` as you are used to!