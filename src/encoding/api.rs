/// `TryFromJson` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a JSON
/// formatted type.
#[cfg(feature = "serde_json")]
pub trait TryFromJson<T>
where
    Self: Sized,
{
    type Error;
    fn try_from_json(_: T) -> Result<Self, Self::Error>;
}

/// `TryIntoJson` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryIntoJson` is automatically implemented for types
/// which implement `TryFromJson`.
#[cfg(feature = "serde_json")]
pub trait TryIntoJson<T> {
    type Error;
    fn try_into_json(self) -> Result<T, Self::Error>;
}

/// `TryFromCbor` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a CBOR
/// formatted type.
#[cfg(feature = "serde_cbor")]
pub trait TryFromCbor<T>
where
    Self: Sized,
{
    type Error;
    fn try_from_cbor(_: T) -> Result<Self, Self::Error>;
}

/// `TryIntoCbor` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryIntoCbor` is automatically implemented for types
/// which implement `TryFromCbor`.
#[cfg(feature = "serde_cbor")]
pub trait TryIntoCbor<T> {
    type Error;
    fn try_into_cbor(self) -> Result<T, Self::Error>;
}

/// `TryFromYaml` defines a fallible conversion trait which is intended
/// to be implemented on a type which may be converted from a Yaml
/// formatted type.
#[cfg(feature = "serde_yaml")]
pub trait TryFromYaml<T>
where
    Self: Sized,
{
    type Error;
    fn try_from_yaml(_: T) -> Result<Self, Self::Error>;
}

/// `TryIntoYaml` is defined in order to mirror the TryFrom and TryInto
/// core traits. `TryIntoYaml` is automatically implemented for types
/// which implement `TryFromYaml`.
#[cfg(feature = "serde_yaml")]
pub trait TryIntoYaml<T> {
    type Error;
    fn try_into_yaml(self) -> Result<T, Self::Error>;
}
