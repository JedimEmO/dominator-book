# The Dominator Book

This book aims to serve as a starting point for learning and exploring the [**DOMINATOR**](https://github.com/Pauan/rust-dominator) framework made by [Pauan](https://github.com/Pauan).
It is still work in progress, but should hopefully give at least some value for the very basics.

[Latest version hosted on github pages](https://jedimemo.github.io/dominator-book/)

## Following the tutorials

To run the tutorials, you will need either `trunk` or `nvm`, and the rust toolchain with the `wasm32-unknown-unknown` target installed.

To install trunk, follow the instructions here: https://trunkrs.dev/guide/getting-started/installation.html
To install NVM, follow the instructions here: https://github.com/nvm-sh/nvm?tab=readme-ov-file#install--update-script

To install rust and the wasm32 target, first install the rust compiler following the instructions at https://www.rustup.rs.

Then, open a new shell and do the following:

```
rustup target add wasm32-unknown-unknown
```

When you have your dependencies installed, go to the tutorial folder under `tutorials` that you wish to run, and do the following:

```shell
trunk serve --open
```

or if you want to get debug symbols in the browser: 

```shell
trunk serve index.debug.html --open
```

If you wish to use the rollup plugin with npm, do the following instead:

```shell
nvm i & nvm use
npm i && npm start --open
```

This should open a browser window with the correct tutorial.
You can now edit code in the `src` folder of the tutorial, and changes should be recompiled and reflected in your browser as you proceed.

## Building the Book

Make sure you have `mdbook` installed:

```shell
cargo install mdbook
```

Then simply run `mdbook serve` from the root of this repository

## Structuring example code

Code that is included in the text of the book as example code should be included from either `src/doc-imports` or one of the tutorial applications under `tutorials/`.
This ensures that we do not end up with stale/uncompilable code in the md-files.

Make sure to not duplicate code between the doc-import and tutorial crates.
Put things in the tutorial folder if it makes sense to show as a standalone tutorial example, otherwise put code snippets in the doc-imports crate.
Within doc imports, rs files should mirror the md files they include samples for to make it easier to find relevant snippets.