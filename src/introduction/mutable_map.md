# Mutable maps

The `MutableBTreeMap` type provides an observable container with map semantics, much the same way `MutableVec` provides
vector semantics.

The ideas are fairly similar to what we already covered for `MutableVec`, but there are more ways to get signals for the
map entries than there are for the mutable vector.

## SignalVec of keys and entries

Let's first look at the `signal_vec_keys()` and `signal_vec_entries()` methods.
They give us a `SignalVec<Item=Key>` and `SignalVec<Item=(Key,Value)>` respectively.
They will only update at indexes corresponding to keys for which a value was inserted, changed or removed in the map.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_map.rs:mutable_map_1}}
```

## Get a signal to the value for a specific key

We sometimes want to show the value for a key, if it is in the map at all.
The `signal_map().key_cloned(key)` method will give us just that; a signal to whatever (if any) value exists in the map
for the specified key.

In the following example, we transform the key_cloned() signal for the key "my-key" to a signal of String, which we
insert into the dom as text.

```rust,no_run,noplayground
{{#include ../doc-imports/src/introduction/mutable_map.rs:mutable_map_key_cloned}}
```