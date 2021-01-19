#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use crate::error::Error;
#[cfg(any(feature = "serde_json", feature = "serde_yaml"))]
use alloc::string::String;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use alloc::vec::Vec;
#[cfg(any(feature = "serde_json", feature = "serde_cbor"))]
use serde::Deserialize;
#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use serde::{de::DeserializeOwned, Serialize};

mod api;
pub use api::*;

#[cfg(feature = "serde_json")]
impl<T, U> TryIntoJson<U> for T
where
    U: TryFromJson<T>,
{
    type Error = U::Error;
    fn try_into_json(self) -> Result<U, U::Error> {
        U::try_from_json(self)
    }
}

#[cfg(feature = "serde_cbor")]
impl<T, U> TryIntoCbor<U> for T
where
    U: TryFromCbor<T>,
{
    type Error = U::Error;
    fn try_into_cbor(self) -> Result<U, U::Error> {
        U::try_from_cbor(self)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T, U> TryIntoYaml<U> for T
where
    U: TryFromYaml<T>,
{
    type Error = U::Error;
    fn try_into_yaml(self) -> Result<U, U::Error> {
        U::try_from_yaml(self)
    }
}

/// JSON
#[cfg(feature = "serde_json")]
impl<'a, T> TryFromJson<&'a str> for T
where
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_json(s: &'a str) -> Result<T, Self::Error> {
        Ok(serde_json::from_str(s)?)
    }
}

#[cfg(feature = "serde_json")]
impl<'a, T> TryFromJson<&'a String> for T
where
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_json(e: &'a String) -> Result<T, Self::Error> {
        Ok(serde_json::from_str(e.as_str())?)
    }
}

#[cfg(feature = "serde_json")]
impl<T> TryFromJson<String> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_json(e: String) -> Result<T, Self::Error> {
        Ok(serde_json::from_str(e.as_str())?)
    }
}

#[cfg(feature = "serde_json")]
impl<'a, T> TryFromJson<&'a [u8]> for T
where
    Self: Sized,
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_json(b: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(b)?)
    }
}

#[cfg(feature = "serde_json")]
impl<'a, T> TryFromJson<&'a Vec<u8>> for T
where
    Self: Sized,
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_json(b: &'a Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(b.as_slice())?)
    }
}

#[cfg(feature = "serde_json")]
impl<T> TryFromJson<Vec<u8>> for T
where
    Self: Sized,
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_json(b: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(b.as_slice())?)
    }
}

/// CBOR
#[cfg(feature = "serde_cbor")]
impl<'a, T> TryFromCbor<&'a [u8]> for T
where
    Self: Sized,
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_cbor(b: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(serde_cbor::from_slice(b)?)
    }
}

#[cfg(feature = "serde_cbor")]
impl<'a, T> TryFromCbor<&'a Vec<u8>> for T
where
    Self: Sized,
    T: Serialize + Deserialize<'a>,
{
    type Error = Error;
    fn try_from_cbor(b: &'a Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_cbor::from_slice(b.as_slice())?)
    }
}

#[cfg(feature = "serde_cbor")]
impl<T> TryFromCbor<Vec<u8>> for T
where
    Self: Sized,
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_cbor(b: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(serde_cbor::from_slice(b.as_slice())?)
    }
}

/// YAML
#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<&str> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(s: &str) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_str(s)?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<&String> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(s: &String) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_str(s.as_str())?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<String> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(s: String) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_str(s.as_str())?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<&[u8]> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(b: &[u8]) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_slice(b)?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<&Vec<u8>> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(b: &Vec<u8>) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_slice(b.as_slice())?)
    }
}

#[cfg(feature = "serde_yaml")]
impl<T> TryFromYaml<Vec<u8>> for T
where
    T: Serialize + DeserializeOwned,
{
    type Error = Error;
    fn try_from_yaml(b: Vec<u8>) -> Result<T, Self::Error> {
        Ok(serde_yaml::from_slice(b.as_slice())?)
    }
}
