# Binary Tree Graph (btree_graph)

This library is a minimal implementation of a graph 
(abstract data structure). 

The primary goals of the implementation are to be 
minimal and idiomatic to the Rust language. The `alloc`
crate is the only dependency when compiled with default
features. As one might assume, `alloc` is required for 
reason the implementation relies on `BTreeMap` (and the
`BTreeSet` wrapper).

Secondary concerns include serialization,
deserialization, and encoding. For these the `serde`,
`serde_json`, `serde_yaml`, and `serde_cbor` crates
are included and available under the feature flags:
`serde`, `json`, `yaml`, and `cbor`. Please see the 
encoding module for notes on the available APIs.
*Note: using `json`, `yaml`, or `cbor` features will
automatically require a `serde` dependency.*

## Example
```rust
use btree_graph::BTreeGraph;

fn main() {
    let mut graph: BTreeGraph<String, String> = BTreeGraph::new();
    // Add nodes.
    graph.add_vertex(String::from("Jim"));
    graph.add_vertex(String::from("Jane"));
    // Add a relationship.
    graph.add_edge(String::from("Jim"), String::from("Jane"), String::from("Loves"));
    
    // Assert relationship now exists.
    assert!(graph.adjacdent(String::from("Jim"), String::from("Jane")));
}

```

## API

Please see the [API](./src/graph/api.rs) for a full list of
available methods.

## License

This work is dually licensed under MIT OR Apache-2.0.