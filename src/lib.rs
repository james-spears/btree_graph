#![no_std]
extern crate alloc;
mod error;
pub use error::Error;
mod graph;
pub use graph::*;
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
pub mod encoding;
