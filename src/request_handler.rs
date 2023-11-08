//! # Request Handler
//! This module contains the request handler for the API calls.

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
/// * `Result<T, reqwest::Error>` - The response from the API.
pub async fn perform_get_request<T>(
    request_builder: RequestBuilder,
    limiter: &DefaultDirectRateLimiter,
) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    limiter.until_ready().await;
    let response = request_builder.send().await?;
    response.json::<T>().await
}
