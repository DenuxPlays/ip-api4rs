use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

use crate::client::{BlockingIpApi, IpApi};
use crate::error::IpApiError;
use crate::model::ip_response::{IpDefaultResponse, IpFullResponse};
use crate::{request_handler, util};

/// A client for the ip-api.com API that blocks the current thread.
pub struct BlockingIpApiClient {
    /// The client to use for the requests.
    pub client: Client,
    /// The rate limiter to use for the requests.
    pub limiter: Option<DefaultDirectRateLimiter>,
    /// The API key to use for the requests.
    pub api_key: Option<String>,
}

impl Default for BlockingIpApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockingIpApiClient {
    /// Creates a new BlockingIpApiClient with no API key.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            limiter: Some(RateLimiter::direct(Quota::per_minute(nonzero!(45u32)))),
            api_key: None,
        }
    }

    /// Creates a new BlockingIpApiClient with an API key.
    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            client: Client::new(),
            limiter: None,
            api_key: Some(api_key),
        }
    }
}

impl IpApi for BlockingIpApiClient {
    fn get_api_key(&self) -> &Option<String> {
        &self.api_key
    }

    fn get_rate_limiter(&self) -> &Option<DefaultDirectRateLimiter> {
        &self.limiter
    }
}

impl BlockingIpApi for BlockingIpApiClient {
    fn query_api_default(&self, ip: &String) -> Result<IpDefaultResponse, IpApiError> {
        let request = util::requests::get_default_blocking_get_request(&ip.to_string(), self);
        request_handler::perform_blocking_get_request::<IpDefaultResponse>(request, &self.limiter)
    }

    fn query_api_fully(&self, ip: &String) -> Result<IpFullResponse, IpApiError> {
        let request = util::requests::get_blocking_get_request::<IpFullResponse>(&ip.to_string(), self);
        request_handler::perform_blocking_get_request::<IpFullResponse>(request, &self.limiter)
    }

    fn query_api<T>(&self, ip: &String) -> Result<T, IpApiError>
    where
        T: DeserializeOwned,
    {
        let request = util::requests::get_blocking_get_request::<T>(&ip.to_string(), self);
        request_handler::perform_blocking_get_request::<T>(request, &self.limiter)
    }

    fn get_http_client(&self) -> &Client {
        &self.client
    }
}
