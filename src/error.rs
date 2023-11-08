//! A module that contains the error type for the library.

use crate::model::ip_response::ErrorResponse;
use thiserror::Error;

/// An enum that contains all the possible errors that can occur when querying the API.
#[derive(Error, Debug)]
pub enum IpApiError {
    #[error("The supplied ip address is in a reserved range.")]
    ReservedRange(ErrorResponse),
    #[error("The supplied ip address is invalid.")]
    InvalidQuery(ErrorResponse),
    #[error("An error occurred while parsing the JSON.")]
    JsonParseError(#[from] serde_json::Error),
    #[error("An error occurred while querying the API.")]
    ReqwestError(#[from] reqwest::Error),
    #[error("An unknown error occurred.")]
    Unknown(String),
}
