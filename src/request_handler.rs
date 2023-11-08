use governor::DefaultDirectRateLimiter;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub async fn perform_get_request<T>(request_builder: RequestBuilder, limiter: &DefaultDirectRateLimiter) -> Result<T, reqwest::Error>
	where T: DeserializeOwned {
	limiter.until_ready().await;
	let response = request_builder.send().await?;
	response.json::<T>().await
}
