//! # Request Handler
//! This module contains the request handler for the API calls.

use crate::constant;
use crate::error::IpApiError;
use crate::model::ip_response::ErrorResponse;
use governor::DefaultDirectRateLimiter;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

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
    limiter: &DefaultDirectRateLimiter,
) -> Result<T, IpApiError>
where
    T: DeserializeOwned,
{
    limiter.until_ready().await;
    let response = request_builder.send().await?;
    let json = response.text().await?;
    if json.contains("\"status\":\"fail\"") {
        return match serde_json::from_str::<ErrorResponse>(&json) {
            Ok(error_response) => match error_response.message.as_str() {
                constant::ERROR_RESERVED_RANGE => Err(IpApiError::ReservedRange(error_response)),
                constant::ERROR_INVALID_QUERY => Err(IpApiError::InvalidQuery(error_response)),
                _ => Err(IpApiError::Unknown(json)),
            },
            Err(err) => Err(IpApiError::JsonParseError(err)),
        };
    }
    match serde_json::from_str::<T>(&json) {
        Ok(response) => Ok(response),
        Err(err) => Err(IpApiError::JsonParseError(err)),
    }
}
