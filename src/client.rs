use governor::DefaultDirectRateLimiter;
#[cfg(feature = "blocking")]
use reqwest::blocking;
use serde::de::DeserializeOwned;
use std::future::Future;

use crate::error::IpApiError;
use crate::model::ip_response::{IpDefaultResponse, IpFullResponse};

/// The main client for the ip-api.com API.
pub trait IpApi {
    /// Gets the optional API key.
    ///
    /// # Returns
    /// * `Option<String>` - The optional API key.
    fn get_api_key(&self) -> &Option<String>;

    /// Gets the rate limiter.
    ///
    /// # Returns
    /// * `&DefaultDirectRateLimiter` - The rate limiter.
    fn get_rate_limiter(&self) -> &Option<DefaultDirectRateLimiter>;
}

/// The blocking client for the ip-api.com API.
#[cfg(feature = "blocking")]
pub trait BlockingIpApi: IpApi {
    /// Queries the API with the default fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpDefaultResponse` - The response from the API.
    fn query_api_default(&self, ip: &str) -> Result<IpDefaultResponse, IpApiError>;

    /// Queries the API with all fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpFullResponse` - The response from the API.
    fn query_api_fully(&self, ip: &str) -> Result<IpFullResponse, IpApiError>;

    /// Queries the API with a custom struct.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    /// * `T` - The custom struct to deserialize the response into.
    ///
    /// # Returns
    /// * `T` - The response from the API.
    fn query_api<T>(&self, ip: &str) -> Result<T, IpApiError>
    where
        T: DeserializeOwned;

    /// Gets you the blocking http client.
    ///
    /// # Returns
    /// * `&reqwest::blocking::Client` - The blocking http client.
    fn get_http_client(&self) -> &blocking::Client;
}

/// The async client for the ip-api.com API.
pub trait AsyncIpApi: IpApi {
    /// Queries the API with the default fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpDefaultResponse` - The response from the API.
    fn query_api_default(&self, ip: &str) -> impl Future<Output = Result<IpDefaultResponse, IpApiError>> + Send;

    /// Queries the API with all fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpFullResponse` - The response from the API.
    fn query_api_fully(&self, ip: &str) -> impl Future<Output = Result<IpFullResponse, IpApiError>> + Send;

    /// Queries the API with a custom struct.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    /// * `T` - The custom struct to deserialize the response into.
    ///
    /// # Returns
    /// * `T` - The response from the API.
    fn query_api<T>(&self, ip: &str) -> impl Future<Output = Result<T, IpApiError>> + Send
    where
        T: DeserializeOwned;
    /// Gets you the async http client.
    ///
    /// # Returns
    /// * `&reqwest::Client` - The async http client.
    fn get_http_client(&self) -> &reqwest::Client;
}
