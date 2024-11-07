//! # Request Handler
//! This module contains the request handler for the API calls.

use crate::constant;
use crate::error::IpApiError;
use crate::model::ip_response::ErrorResponse;
use governor::DefaultDirectRateLimiter;
#[cfg(feature = "blocking")]
use reqwest::blocking;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
#[cfg(feature = "blocking")]
use std::thread::sleep;
#[cfg(feature = "blocking")]
use std::time::Duration;

/// Performs a GET request to the API.
///
/// # Arguments
/// * `request_builder` - The request builder to use.
/// * `limiter` - The rate limiter to use.
///
/// # Returns
/// * `Result<T, ip-api4rs::error::IpApiError>` - The response from the API.
pub async fn perform_get_request<T>(
    request_builder: RequestBuilder,
    limiter: &Option<DefaultDirectRateLimiter>,
) -> Result<T, IpApiError>
where
    T: DeserializeOwned,
{
    wait_for_rate_limiter(limiter).await;
    let response = request_builder.send().await?;
    let json = response.text().await?;
    process_result(json)
}

/// Performs a blocking GET request to the API.
///
/// # Arguments
/// * `request_builder` - The request builder to use.
/// * `limiter` - The rate limiter to use.
///
/// # Returns
/// * `Result<T, ip-api4rs::error::IpApiError>` - The response from the API.
#[cfg(feature = "blocking")]
pub fn perform_blocking_get_request<T>(
    request_builder: blocking::RequestBuilder,
    limiter: &Option<DefaultDirectRateLimiter>,
) -> Result<T, IpApiError>
where
    T: DeserializeOwned,
{
    block_until_rate_limiter(limiter);
    let response = request_builder.send()?;
    let json = response.text()?;
    process_result::<T>(json)
}

/// Processes the result from the API.
/// Checks for errors and parses the result.
///
/// # Arguments
/// * `json` - The json to parse.
///
/// # Returns
/// * `Result<T, ip-api4rs::error::IpApiError>` - The parsed result.
fn process_result<T>(json: String) -> Result<T, IpApiError>
where
    T: DeserializeOwned,
{
    if let Some(error) = validate_result(json.clone()) {
        return Err(error);
    }
    parse_result::<T>(&json)
}

/// Validates the result for errors.
///
/// # Arguments
/// * `json` - The json to parse.
///
/// # Returns
/// * `Option<IpApiError>` - The error if there is one.
fn validate_result(json: String) -> Option<IpApiError> {
    if json.contains("\"status\":\"fail\"") {
        return match serde_json::from_str::<ErrorResponse>(&json) {
            Ok(error_response) => match error_response.message.as_str() {
                constant::ERROR_RESERVED_RANGE => Some(IpApiError::ReservedRange(error_response)),
                constant::ERROR_INVALID_QUERY => Some(IpApiError::InvalidQuery(error_response)),
                _ => Some(IpApiError::Unknown(json.clone())),
            },
            Err(err) => Some(IpApiError::JsonParseError(err)),
        };
    }
    None
}

/// Parses the result from the API.
///
/// # Arguments
/// * `json` - The json to parse.
///
/// # Returns
/// * `Result<T, ip-api4rs::error::IpApiError>` - The parsed result.
fn parse_result<T>(json: &str) -> Result<T, IpApiError>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<T>(json) {
        Ok(response) => Ok(response),
        Err(err) => Err(IpApiError::JsonParseError(err)),
    }
}

/// Waits for the rate limiter to be ready.
async fn wait_for_rate_limiter(limiter: &Option<DefaultDirectRateLimiter>) {
    if let Some(limiter) = limiter {
        limiter.until_ready().await
    }
}

/// Blocks until the rate limiter is ready.
#[cfg(feature = "blocking")]
fn block_until_rate_limiter(limiter: &Option<DefaultDirectRateLimiter>) {
    if let Some(limiter) = limiter {
        while limiter.check().is_err() {
            sleep(Duration::new(1, 0));
        }
    }
}
