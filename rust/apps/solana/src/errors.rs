use alloc::format;
use alloc::string::{String, ToString};
use bitcoin::base58::Error as Base58Error;
use keystore::errors::KeystoreError;
use serde_json::Error;
use thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("meet error when encoding address: {0}")]
    AddressError(String),
    #[error("keystore operation failed, reason: {0}")]
    KeystoreError(String),

    #[error("Program `{0}` is not supported yet")]
    UnsupportedProgram(String),

    #[error("Meet invalid data when reading `{0}`")]
    InvalidData(String),

    #[error("Error occurred when parsing program instruction, reason: `{0}`")]
    ProgramError(String),

    #[error("Could not found account for `{0}`")]
    AccountNotFound(String),

    #[error("Could not parse transaction, reason: `{0}`")]
    ParseTxError(String),
}

pub type Result<T> = core::result::Result<T, SolanaError>;

impl From<Base58Error> for SolanaError {
    fn from(value: Base58Error) -> Self {
        Self::AddressError(format!("base58Error: {value}"))
    }
}

impl From<KeystoreError> for SolanaError {
    fn from(value: KeystoreError) -> Self {
        Self::KeystoreError(value.to_string())
    }
}

impl From<hex::FromHexError> for SolanaError {
    fn from(value: hex::FromHexError) -> Self {
        Self::InvalidData(format!("hex operation failed {value}"))
    }
}

impl From<serde_json::Error> for SolanaError {
    fn from(value: Error) -> Self {
        Self::ParseTxError(format!(
            "serde json operation failed {:?}",
            value.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_variants_have_actionable_messages() {
        let errors = [
            SolanaError::AddressError("bad address".to_string()),
            SolanaError::KeystoreError("locked".to_string()),
            SolanaError::UnsupportedProgram("program".to_string()),
            SolanaError::InvalidData("message".to_string()),
            SolanaError::ProgramError("invalid instruction".to_string()),
            SolanaError::AccountNotFound("source".to_string()),
            SolanaError::ParseTxError("invalid transaction".to_string()),
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn external_errors_keep_their_context() {
        let hex_error = hex::decode("xyz").unwrap_err();
        assert!(SolanaError::from(hex_error)
            .to_string()
            .contains("hex operation failed"));

        let json_error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        assert!(SolanaError::from(json_error)
            .to_string()
            .contains("serde json operation failed"));
    }
}
