//! Encoding types used by Sui.
//!
//! This module is mostly to avoid importing [`fastcrypto`](https://github.com/MystenLabs/fastcrypto)

use base64::Engine;
use base64::engine::GeneralPurpose;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::base64::Base64;
use serde_with::{DeserializeAs, SerializeAs, formats};
use sui_sdk_types::bcs::{FromBcs, ToBcs};

/// Default encoder for Base64 data.
pub(crate) const BASE64_DEFAULT_ENGINE: GeneralPurpose = GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::general_purpose::PAD,
);

/// Convenience method for decoding base64 bytes the way Sui expects.
pub fn decode_base64_default(value: impl AsRef<[u8]>) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64_DEFAULT_ENGINE.decode(value)
}

/// Convenience method for encoding bytes to base64 the way Sui expects.
pub fn encode_base64_default(value: impl AsRef<[u8]>) -> String {
    BASE64_DEFAULT_ENGINE.encode(value)
}

// =============================================================================
//  Base64Bcs
// =============================================================================

/// Serialize values with base64-encoded BCS.
///
/// The type serializes a value as a base64 string of its BCS encoding.
/// It works on any type compatible with [`bcs`] for (de)serialization.
pub struct Base64Bcs<Alphabet = serde_with::base64::Standard, Padding = formats::Padded>(
    std::marker::PhantomData<(Alphabet, Padding)>,
)
where
    Alphabet: serde_with::base64::Alphabet,
    Padding: formats::Format;

impl<'de, T, Alphabet, Padding> DeserializeAs<'de, T> for Base64Bcs<Alphabet, Padding>
where
    Alphabet: serde_with::base64::Alphabet,
    Padding: formats::Format,
    T: for<'a> Deserialize<'a>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Base64::<Alphabet, Padding>::deserialize_as(deserializer)?;
        FromBcs::from_bcs(&bytes).map_err(serde::de::Error::custom)
    }
}

impl<T, Alphabet> SerializeAs<T> for Base64Bcs<Alphabet, formats::Padded>
where
    Alphabet: serde_with::base64::Alphabet,
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = source.to_bcs().map_err(serde::ser::Error::custom)?;
        Base64::<Alphabet, formats::Padded>::serialize_as(&bytes, serializer)
    }
}

impl<T, Alphabet> SerializeAs<T> for Base64Bcs<Alphabet, formats::Unpadded>
where
    Alphabet: serde_with::base64::Alphabet,
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = source.to_bcs().map_err(serde::ser::Error::custom)?;
        Base64::<Alphabet, formats::Unpadded>::serialize_as(&bytes, serializer)
    }
}

// =============================================================================
//  Base58
// =============================================================================

/// Base58 encoding format. Can be used with [`serde_with::serde_as`].
pub struct Base58;

impl Base58 {
    pub fn decode(data: impl AsRef<[u8]>) -> Result<Vec<u8>, bs58::decode::Error> {
        bs58::decode(data).into_vec()
    }

    pub fn encode(data: impl AsRef<[u8]>) -> String {
        bs58::encode(data).into_string()
    }
}

impl<'de, T> DeserializeAs<'de, T> for Base58
where
    T: TryFrom<Vec<u8>>,
    T::Error: std::fmt::Debug,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = Self::decode(&s).map_err(serde::de::Error::custom)?;
        let length = value.len();
        value
            .try_into()
            .map_err(|error| serde::de::Error::custom(BytesConversionError { length, error }))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Converting from a Byte Vector of length {length}: {error:?}")]
pub struct BytesConversionError<E> {
    pub length: usize,
    pub error: E,
}

impl<T> SerializeAs<T> for Base58
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::encode(value).serialize(serializer)
    }
}
