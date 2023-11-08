// #![deny(missing_docs)]

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::Error;
use serde::de::DeserializeOwned;

use model::ip_response::{IpDefaultResponse, IpFullResponse};

pub mod constant;
pub mod model;
pub mod request_handler;
pub mod util;

pub struct IpApiClient {
    pub client: reqwest::Client,
    pub limiter: DefaultDirectRateLimiter,
    pub api_key: Option<String>,
}

impl Default for IpApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl IpApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            limiter: RateLimiter::direct(Quota::per_minute(nonzero!(45u32))),
            api_key: None,
        }
    }

    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            limiter: RateLimiter::direct(Quota::per_minute(nonzero!(45u32))),
            api_key: Some(api_key),
        }
    }

    pub async fn query_api_default(&self, ip: &String) -> Result<IpDefaultResponse, Error> {
        let request = util::requests::build_default_get_request(&ip.to_string(), self);
        request_handler::perform_get_request::<IpDefaultResponse>(request, &self.limiter).await
    }

    pub async fn query_api_fully(&self, ip: &String) -> Result<IpFullResponse, Error> {
        let request = util::requests::build_get_request::<IpFullResponse>(&ip.to_string(), self);
        request_handler::perform_get_request::<IpFullResponse>(request, &self.limiter).await
    }

    pub async fn query_api<T>(&self, ip: &String) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let request = util::requests::build_get_request::<T>(&ip.to_string(), self);
        request_handler::perform_get_request::<T>(request, &self.limiter).await
    }
}

#[cfg(test)]
mod test {
	use crate::IpApiClient;
	use crate::model::ip_response::{IpDefaultResponse, IpFullResponse};
    use crate::util::urls::{build_http_url_from_struct, build_https_url_from_struct, build_url_without_fields};

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
        assert_eq!(url, format!("http://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting", TEST_IP));
    }

	#[test]
	fn test_custom_https_url() {
		let url = build_https_url_from_struct::<IpFullResponse>(&TEST_IP.to_string());
		assert_eq!(url, format!("https://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting", TEST_IP));
	}

	#[tokio::test]
	async fn test_client() {
		let client = IpApiClient::new();
		assert_eq!(client.api_key, None);
		let result = client.query_api_default(&EXTERN_TEST_IP.to_string()).await;
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
		let result = client.query_api::<IpDefaultResponse>(&EXTERN_TEST_IP.to_string()).await;
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
