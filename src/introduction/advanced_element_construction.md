# Advanced Element Construction

Let's take a look at a few slightly more advanced way of setting up elements using the `html!` macro.

## Accessing the real DOM node

If we wish to gain access to the underlying DOM node, we have to use the `with_node!` macro provided by **DOMINATOR**.

With this, we gain direct access to all methods on the actual DOM node.

```rust
{{#include ../doc-imports/src/introduction/advanced_element_construction.rs:with_node_example}}
```

By using the `with_node!` macro in the html macros method block, we gain access to the DOM node reference inside the `with_node`s apply block.

A bit more concretely; the `.future()` and `.apply()` methods are applied to our newly constructed `div` builder as usual, but inside the async body we have access to the element reference to call blur on it when the signal future resolves!

Note that the apply function allows us to access the DOM node reference before it is inserted into the DOM tree.

## Constructing elements with multiple child nodes

Very often, we need to create an element with several child nodes.
We may also need to mix text node with html element nodes in the child list.
Some children may be dynamically created based on the content of a signal, with static siblings.

Luckily, the `html!` macro makes this trivial to do from **DOMINATOR**!

Take a look at this multinode example:

```rust
{{#include ../doc-imports/src/introduction/advanced_element_construction.rs:multinode}}
```

As you can see in the method block of the macro, we simply make multiple calls to the various child-inserting methods.
This particular example expands into the following DOM structure:

```rust
{{#include ../doc-imports/src/introduction/advanced_element_construction.rs:multinode_expanded}}
```

This even works with `fragments` and `children_vec`, which may expand into several dynamically allocated children:

```rust
{{#include ../doc-imports/src/introduction/advanced_element_construction.rs:multinode_multichild}}
```