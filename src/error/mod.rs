#[cfg(feature = "fmt")]
use core::fmt::{Display, Formatter, Result};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod test;

#[cfg(all(feature = "fmt", feature = "serde_cbor"))]
static CBOR_ERROR: &str = "Encoding Error: Serializing or deserializing CBOR";
#[cfg(all(feature = "fmt", feature = "serde_json"))]
static JSON_ERROR: &str = "Encoding Error: Serializing or deserializing JSON";
#[cfg(all(feature = "fmt", feature = "serde_yaml"))]
static YAML_ERROR: &str = "Encoding Error: Serializing or deserializing YAML";

/// Errors which may occur during serializing and deserializing the graph
/// data structure.
#[derive(PartialEq, Debug)]
#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EncodingError {
    #[cfg(feature = "serde_cbor")]
    CborError,
    #[cfg(feature = "serde_json")]
    JsonError,
    #[cfg(feature = "serde_yaml")]
    YamlError,
}

#[cfg(all(
    feature = "fmt",
    any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml")
))]
impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            #[cfg(feature = "serde_cbor")]
            EncodingError::CborError => write!(f, "{}", CBOR_ERROR),
            #[cfg(feature = "serde_json")]
            EncodingError::JsonError => write!(f, "{}", JSON_ERROR),
            #[cfg(feature = "serde_yaml")]
            EncodingError::YamlError => write!(f, "{}", YAML_ERROR),
        }
    }
}

#[cfg(feature = "serde_cbor")]
impl From<serde_cbor::Error> for EncodingError {
    fn from(_: serde_cbor::Error) -> EncodingError {
        EncodingError::CborError
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for EncodingError {
    fn from(_: serde_json::Error) -> EncodingError {
        EncodingError::JsonError
    }
}

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for EncodingError {
    fn from(_: serde_yaml::Error) -> EncodingError {
        EncodingError::YamlError
    }
}

#[cfg(feature = "fmt")]
static VERTEX_DOES_NOT_EXIST_ERROR: &str = "graph Error: Vertex does not exist";

/// Errors which may occur during normal usage of the library.
#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    VertexDoesNotExist,
    #[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
    EncodingError(EncodingError),
}

#[cfg(feature = "fmt")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::VertexDoesNotExist => write!(f, "{}", VERTEX_DOES_NOT_EXIST_ERROR),
            #[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
            Error::EncodingError(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
impl From<EncodingError> for Error {
    fn from(e: EncodingError) -> Error {
        Error::EncodingError(e)
    }
}

#[cfg(feature = "serde_cbor")]
impl From<serde_cbor::Error> for Error {
    fn from(e: serde_cbor::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "serde_yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}
