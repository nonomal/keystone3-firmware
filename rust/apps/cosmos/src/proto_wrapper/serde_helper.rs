use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn from_base64(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(s)
}

pub fn to_base64<T: AsRef<[u8]>>(input: T) -> String {
    base64::encode(&input)
}

pub mod base64_format {
    use super::*;
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    use super::{from_base64, to_base64};

    pub fn serialize<S, T>(data: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        serializer.serialize_str(&to_base64(data))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: From<Vec<u8>>,
    {
        let s = String::deserialize(deserializer)?;
        from_base64(&s)
            .map_err(|err| de::Error::custom(err.to_string()))
            .map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Base64Value {
        #[serde(with = "base64_format")]
        data: Vec<u8>,
    }

    #[test]
    fn base64_helpers_roundtrip_binary_data() {
        let data = vec![0x00, 0x01, 0xFE, 0xFF];
        let encoded = to_base64(&data);

        assert_eq!(encoded, "AAH+/w==");
        assert_eq!(from_base64(&encoded).unwrap(), data);
    }

    #[test]
    fn serde_base64_format_roundtrip_and_rejects_invalid_input() {
        let value = Base64Value {
            data: vec![1, 2, 3, 4],
        };
        let json = serde_json::to_string(&value).unwrap();

        assert_eq!(json, r#"{"data":"AQIDBA=="}"#);
        assert_eq!(serde_json::from_str::<Base64Value>(&json).unwrap(), value);
        assert!(serde_json::from_str::<Base64Value>(r#"{"data":"%%%"}"#).is_err());
    }
}
