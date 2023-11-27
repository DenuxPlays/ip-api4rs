//! # ip-api-rust
//!
//! A simple Rust wrapper for the [ip-api.com](https://ip-api.com) API.

use async_trait::async_trait;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::client::{AsyncIpApi, IpApi};
use crate::error::IpApiError;
use model::ip_response::{IpDefaultResponse, IpFullResponse};

#[cfg(feature = "blocking")]
pub mod blocking;
mod client;
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

#[cfg(test)]
mod test {
    use crate::blocking::client::BlockingIpApiClient;
    use crate::client::{AsyncIpApi, BlockingIpApi};
    use crate::error::IpApiError;
    use crate::model::ip_response::{IpDefaultResponse, IpFullResponse};
    use crate::util::urls::{build_http_url_from_struct, build_https_url_from_struct, build_url_without_fields};
    use crate::IpApiClient;

    const TEST_IP: &str = "127.0.0.1";
    const EXTERN_TEST_IP: &str = "8.8.8.8";

    #[test]
    fn test_default_http_url() {
        let url = build_url_without_fields(false, &TEST_IP.to_string());
        assert_eq!(url, format!("http://ip-api.com/json/{}", TEST_IP))
    }

    #[test]
    fn test_default_https_url() {
        let url = build_url_without_fields(true, &TEST_IP.to_string());
        assert_eq!(url, format!("https://ip-api.com/json/{}", TEST_IP))
    }

    #[test]
    fn test_custom_http_url() {
        let url = build_http_url_from_struct::<IpFullResponse>(&TEST_IP.to_string());
        assert_eq!(url, format!("http://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,message", TEST_IP));
    }

    #[test]
    fn test_custom_https_url() {
        let url = build_https_url_from_struct::<IpFullResponse>(&TEST_IP.to_string());
        assert_eq!(url, format!("https://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,message", TEST_IP));
    }

    #[tokio::test]
    async fn test_client() {
        let client = IpApiClient::new();
        assert_eq!(client.api_key, None);
        let result = client.query_api_default(EXTERN_TEST_IP).await;
        let expected = IpDefaultResponse {
            query: EXTERN_TEST_IP.to_string(),
            status: "success".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "VA".to_string(),
            region_name: "Virginia".to_string(),
            city: "Ashburn".to_string(),
            zip: "20149".to_string(),
            lat: 39.03,
            lon: -77.5,
            timezone: "America/New_York".to_string(),
            isp: "Google LLC".to_string(),
            org: "Google Public DNS".to_string(),
            as_number: "AS15169 Google LLC".to_string(),
        };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_client_custom() {
        let client = IpApiClient::new();
        assert_eq!(client.api_key, None);
        let result = client.query_api::<IpDefaultResponse>(EXTERN_TEST_IP).await;
        let expected = IpDefaultResponse {
            query: EXTERN_TEST_IP.to_string(),
            status: "success".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "VA".to_string(),
            region_name: "Virginia".to_string(),
            city: "Ashburn".to_string(),
            zip: "20149".to_string(),
            lat: 39.03,
            lon: -77.5,
            timezone: "America/New_York".to_string(),
            isp: "Google LLC".to_string(),
            org: "Google Public DNS".to_string(),
            as_number: "AS15169 Google LLC".to_string(),
        };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[tokio::test]
    async fn test_error_reserved_range() {
        let client = IpApiClient::new();
        let result = client.query_api::<IpDefaultResponse>("127.0.0.1").await;
        match result.err().unwrap() {
            IpApiError::ReservedRange(error_response) => {
                assert_eq!(error_response.message, "reserved range");
            }
            _ => panic!("Wrong error type returned."),
        }
    }

    #[tokio::test]
    async fn test_error_invalid_query() {
        let client = IpApiClient::new();
        let result = client.query_api::<IpDefaultResponse>("Invalid Query").await;
        match result.err().unwrap() {
            IpApiError::InvalidQuery(error_response) => {
                assert_eq!(error_response.message, "invalid query");
            }
            _ => panic!("Wrong error type returned."),
        }
    }

    #[test]
    fn test_blocking_client() {
        let client = BlockingIpApiClient::new();
        assert_eq!(client.api_key, None);
        let result = client.query_api_default(EXTERN_TEST_IP);
        let expected = IpDefaultResponse {
            query: EXTERN_TEST_IP.to_string(),
            status: "success".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "VA".to_string(),
            region_name: "Virginia".to_string(),
            city: "Ashburn".to_string(),
            zip: "20149".to_string(),
            lat: 39.03,
            lon: -77.5,
            timezone: "America/New_York".to_string(),
            isp: "Google LLC".to_string(),
            org: "Google Public DNS".to_string(),
            as_number: "AS15169 Google LLC".to_string(),
        };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_blocking_client_custom() {
        let client = BlockingIpApiClient::new();
        assert_eq!(client.api_key, None);
        let result = client.query_api::<IpDefaultResponse>(EXTERN_TEST_IP);
        let expected = IpDefaultResponse {
            query: EXTERN_TEST_IP.to_string(),
            status: "success".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "VA".to_string(),
            region_name: "Virginia".to_string(),
            city: "Ashburn".to_string(),
            zip: "20149".to_string(),
            lat: 39.03,
            lon: -77.5,
            timezone: "America/New_York".to_string(),
            isp: "Google LLC".to_string(),
            org: "Google Public DNS".to_string(),
            as_number: "AS15169 Google LLC".to_string(),
        };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
