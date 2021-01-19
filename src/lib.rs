#![no_std]
extern crate alloc;
pub mod error;
mod graph;
pub use graph::*;
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
pub mod encoding;
