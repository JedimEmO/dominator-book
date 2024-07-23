# Testing DOMINATOR UI code

Testing is an essential part of any serious software project, UIs included.
Our **DOMINATOR** UI code cannot execute directly in an x86 unit test environment though, as it relies heavily upon browser APIs being present to function.

Luckily, wasm-bindgen provide a testing tool that allows us to execute tests compiled to wasm inside a headless browser, essentially providing us with a unit test-like environment to verify our UI code in!

To get a better overview of the wasm-bindgen-test crate, head over to [their official book](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html) to get started.

## Requirements

To recap the required configuration, you will need to set up a few things in your project.

First of all, we have to add wasm-bindgen-tests as a dev-dependency to our crate:

```toml
[dev-dependencies]
wasm-bindgen-test = "x.y.z" # Please use the latest compatible version you can
```

Now, to configure the test environment to be a browser, we need to add the following to the root of our crate (usually lib.rs)

```rust
use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);
```

We also need to install the `wasm-bindgen-cli` tool, and instruct cargo to use it to execute our tests for the wasm32 target.
Make sure that the x.y-z version of wasm-bindgen-cli matches the version of wasm-bindgen you are using in your projects Cargo.toml. 

```shell
cargo install wasm-bindgen-cli --vers x.y.z
```

Then add the following to the `.cargo/config.toml` file in your project:

```toml
[target.wasm32-unknown-unknown]
runner = "wasm-bindgen-test-runner"
```

Lastly, you will need to install either `chromedriver` or `geckodriver` to execute your tests, depending on your browser choice.

## Running the tests

To run your headless tests, the following commands should work:

```shell
CHROMEDRIVER=/path/to/chromedriver cargo test --target wasm32-unknown-unknown
GECKODRIVER=/path/to/geckodriver cargo test --target wasm32-unknown-unknown
```

If you wish to run the tests in a headed browser, i.e. in a browser instance opened in a new window, you can set the `NO_HEADLESS=1` environment variable:


```shell
NO_HEADLESS=1 CHROMEDRIVER=/path/to/chromedriver cargo test --target wasm32-unknown-unknown
```

## An example component with associated in-browser test

Here's a full example of a component with a corresponding unit test.
It's not a very complex component, but it serves to illustrate how we can test that some interaction results in the expected dom structure.
In this case, we make sure that clicking the button 3 times, results in 3 rows of a specific class being added to the DOM.

```rust,no_run,noplayground
{{#include ../doc-imports/src/techniques_and_patterns/in_browser_testing.rs:test}}
```