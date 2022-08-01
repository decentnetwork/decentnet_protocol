use crate::address::AddressError;
use crate::address::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid JSON: `{0}`")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Error parsing address: `{0}`")]
    ParseError(#[from] ParseError),
    #[error("Error doing something with address: `{0}`")]
    AddressError(#[from] AddressError),
    #[error("Error decoding base64 `{0}`")]
    Base64Decode(#[from] base64::DecodeError),
}
