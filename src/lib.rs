#![no_std]
extern crate alloc;

#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
pub mod encoding;
pub mod error;

mod graph;
pub use graph::*;