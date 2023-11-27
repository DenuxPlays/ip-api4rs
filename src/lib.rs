//! # IP-API4rs
//!
//! A simple Rust crate for the [ip-api.com](https://ip-api.com) API.

use async_trait::async_trait;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::Client;
use serde::de::DeserializeOwned;

use model::ip_response::{IpDefaultResponse, IpFullResponse};

use crate::client::{AsyncIpApi, IpApi};
use crate::error::IpApiError;

#[cfg(feature = "blocking")]
pub mod blocking;
pub mod client;
pub mod constant;
pub mod error;
pub mod model;
pub mod request_handler;
pub mod util;

/// The main client for the ip-api.com API.
pub struct IpApiClient {
    /// The client to use for the requests.
    pub client: Client,
    /// The rate limiter to use for the requests.
    pub limiter: Option<DefaultDirectRateLimiter>,
    /// The API key to use for the requests.
    pub api_key: Option<String>,
}

impl Default for IpApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl IpApiClient {
    /// Creates a new BlockingIpApiClient with no API key.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            limiter: Some(RateLimiter::direct(Quota::per_minute(nonzero!(45u32)))),
            api_key: None,
        }
    }

    /// Creates a new IpApiClient with an API key.
    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            client: Client::new(),
            limiter: None,
            api_key: Some(api_key),
        }
    }
}

impl IpApi for IpApiClient {
    fn get_api_key(&self) -> &Option<String> {
        &self.api_key
    }

    fn get_rate_limiter(&self) -> &Option<DefaultDirectRateLimiter> {
        &self.limiter
    }
}

#[async_trait]
impl AsyncIpApi for IpApiClient {
    async fn query_api_default(&self, ip: &str) -> Result<IpDefaultResponse, IpApiError> {
        let request = util::requests::get_default_async_get_request(&ip.to_string(), self);
        request_handler::perform_get_request::<IpDefaultResponse>(request, &self.limiter).await
    }

    async fn query_api_fully(&self, ip: &str) -> Result<IpFullResponse, IpApiError> {
        let request = util::requests::get_async_request::<IpFullResponse>(&ip.to_string(), self);
        request_handler::perform_get_request::<IpFullResponse>(request, &self.limiter).await
    }

    async fn query_api<T>(&self, ip: &str) -> Result<T, IpApiError>
    where
        T: DeserializeOwned,
    {
        let request = util::requests::get_async_request::<T>(&ip.to_string(), self);
        request_handler::perform_get_request::<T>(request, &self.limiter).await
    }

    fn get_http_client(&self) -> &Client {
        &self.client
    }
}
