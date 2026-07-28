use alloc::fmt::Debug;
use alloc::fmt::Formatter;
use alloc::fmt::Result as FmtResult;
use alloc::format;
use alloc::str::FromStr;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use hex;
use thiserror;
use thiserror::Error;

/// Wrapper type around Bytes to deserialize/serialize "0x" prefixed ethereum hex strings
#[derive(Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Bytes(
    #[serde(
        serialize_with = "serialize_bytes",
        deserialize_with = "deserialize_bytes"
    )]
    pub bytes::Bytes,
);

impl Bytes {
    #[inline]
    pub const fn new() -> Self {
        Self(bytes::Bytes::new())
    }

    #[inline]
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Self(bytes::Bytes::from_static(bytes))
    }

    fn hex_encode(&self) -> String {
        hex::encode(self.0.as_ref())
    }
}

impl From<bytes::Bytes> for Bytes {
    fn from(src: bytes::Bytes) -> Self {
        Self(src)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(src: Vec<u8>) -> Self {
        Self(src.into())
    }
}

#[derive(Debug, Clone, Error)]
#[error("Failed to parse bytes: {0}")]
pub struct ParseBytesError(String);

impl FromStr for Bytes {
    type Err = ParseBytesError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(value) = value.strip_prefix("0x") {
            hex::decode(value)
        } else {
            hex::decode(value)
        }
        .map(Into::into)
        .map_err(|e| ParseBytesError(format!("Invalid hex: {e}")))
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Bytes(0x{})", self.hex_encode())
    }
}

pub fn serialize_bytes<S, T>(x: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    s.serialize_str(&format!("0x{}", hex::encode(x.as_ref())))
}

pub fn deserialize_bytes<'de, D>(d: D) -> Result<bytes::Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(d)?;
    if let Some(value) = value.strip_prefix("0x") {
        hex::decode(value)
    } else {
        hex::decode(&value)
    }
    .map(Into::into)
    .map_err(|e| serde::de::Error::custom(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn bytes_parse_debug_and_conversions() {
        let prefixed: Bytes = "0xdeadbeef".parse().unwrap();
        let unprefixed: Bytes = "deadbeef".parse().unwrap();

        assert_eq!(prefixed, unprefixed);
        assert_eq!(format!("{prefixed:?}"), "Bytes(0xdeadbeef)");
        assert_eq!(Bytes::new(), Bytes::default());
        assert_eq!(Bytes::from(vec![1, 2]).0.as_ref(), &[1, 2]);
        assert_eq!(
            Bytes::from(bytes::Bytes::from_static(&[3, 4])).0.as_ref(),
            &[3, 4]
        );
        assert_eq!(Bytes::from_static(&[5, 6]).0.as_ref(), &[5, 6]);
        assert!("0xnot-hex".parse::<Bytes>().is_err());
    }

    #[test]
    fn bytes_serde_accepts_prefixed_and_unprefixed_hex() {
        let value = Bytes::from(vec![0x12, 0xAB]);

        assert_eq!(serde_json::to_string(&value).unwrap(), r#""0x12ab""#);
        assert_eq!(serde_json::from_str::<Bytes>(r#""0x12ab""#).unwrap(), value);
        assert_eq!(serde_json::from_str::<Bytes>(r#""12ab""#).unwrap(), value);
        assert!(serde_json::from_str::<Bytes>(r#""xyz""#).is_err());
    }
}
