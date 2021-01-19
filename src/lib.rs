#![no_std]
extern crate alloc;

/// `encoding` module contains the definition of the encoding API and auto implementations.
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
pub mod encoding;

/// `error` module contains the definition of the Error struct.
pub mod error;

mod graph;
pub use graph::*;