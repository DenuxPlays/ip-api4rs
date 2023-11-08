// #![deny(missing_docs)]

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::Error;
use serde::de::DeserializeOwned;

use model::ip_response::{IpDefaultResponse, IpFullResponse};

pub mod constant;
pub mod model;
pub mod util;
pub mod request_handler;

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
		where T: DeserializeOwned {
		let request = util::requests::build_get_request::<T>(&ip.to_string(), self);
		request_handler::perform_get_request::<T>(request, &self.limiter).await
	}
}

#[cfg(test)]
mod test {

}
