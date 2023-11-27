//! A utility module for building requests to the API.

#[cfg(feature = "blocking")]
use crate::client::BlockingIpApi;
use crate::client::{AsyncIpApi, IpApi};
#[cfg(feature = "blocking")]
use reqwest::blocking;
use reqwest::header::HeaderMap;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::model::ip_response::IpDefaultResponse;
use crate::util::urls::build_url_from_struct;

/// Builds the request for a default response.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `AsyncIpApi` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
pub fn get_default_async_get_request(query: &String, client: &impl AsyncIpApi) -> RequestBuilder {
    get_async_request::<IpDefaultResponse>(&query, client)
}

/// Builds the blocking request for a default response.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `BlockingIpApi` to use for the request.
///
/// # Returns
/// A `blocking::RequestBuilder` for the request.
pub fn get_default_blocking_get_request(query: &String, client: &impl BlockingIpApi) -> blocking::RequestBuilder {
    get_blocking_get_request::<IpDefaultResponse>(&query, client)
}

/// Builds the async request for a custom response.
/// The given struct must implement `DeserializeOwned`.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `AsyncIpApi` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
pub fn get_async_request<T>(query: &String, client: &impl AsyncIpApi) -> RequestBuilder
where
    T: DeserializeOwned,
{
    let url = get_url::<T>(query, client);
    match client.get_api_key() {
        Some(_) => build_https_get_request(&url, client.get_api_key().as_ref().unwrap(), &client.get_http_client()),
        None => build_http_get_request(&url, &client.get_http_client()),
    }
}

/// Builds the blocking request for a custom response.
/// The given struct must implement `DeserializeOwned`.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `BlockingIpApi` to use for the request.
///
/// # Returns
/// A `blocking::RequestBuilder` for the request.
#[cfg(feature = "blocking")]
pub fn get_blocking_get_request<T>(query: &String, client: &impl BlockingIpApi) -> blocking::RequestBuilder
where
    T: DeserializeOwned,
{
    let url = get_url::<T>(query, client);
    match client.get_api_key() {
        Some(_) => {
            build_blocking_https_get_request(&url, client.get_api_key().as_ref().unwrap(), client.get_http_client())
        }
        None => build_blocking_http_get_request(&url, client.get_http_client()),
    }
}

/// Builds a https request.
///
/// # Arguments
/// * `url` - The url to build the request for.
/// * `api_key` - The API key to use for the request.
/// * `client` - The `Client` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
fn build_https_get_request(url: &String, api_key: &String, client: &Client) -> RequestBuilder {
    client.get(url).headers(build_https_header(api_key))
}

/// Builds a blocking https request.
///
/// # Arguments
/// * `url` - The url to build the request for.
/// * `api_key` - The API key to use for the request.
/// * `client` - The `blocking::Client` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
#[cfg(feature = "blocking")]
fn build_blocking_https_get_request(
    url: &String,
    api_key: &String,
    client: &blocking::Client,
) -> blocking::RequestBuilder {
    client.get(url).headers(build_https_header(api_key))
}

/// Builds a http request.
///
/// # Arguments
/// * `url` - The url to build the request for.
/// * `client` - The `Client` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
fn build_http_get_request(url: &String, client: &Client) -> RequestBuilder {
    client.get(url).headers(build_http_header())
}

/// Builds a blocking http request.
///
/// # Arguments
/// * `url` - The url to build the request for.
/// * `client` - The `blocking::Client` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
#[cfg(feature = "blocking")]
fn build_blocking_http_get_request(url: &String, client: &blocking::Client) -> blocking::RequestBuilder {
    client.get(url).headers(build_http_header())
}

/// Builds the url for a request.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `IpApiClient` to use for the request.
///
/// # Returns
/// A `String` containing the url for the request.
fn get_url<T>(query: &String, client: &impl IpApi) -> String
where
    T: DeserializeOwned,
{
    match client.get_api_key() {
        Some(_) => build_url_from_struct::<T>(true, query),
        None => build_url_from_struct::<T>(false, query),
    }
}

/// Builds an header that set the `Accept` and `User-Agent` fields.
///
/// # Returns
/// A `HeaderMap` containing the headers.
fn build_http_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    //headers.insert("User-Agent", format!("ip-api-rust/{}", env!("CARGO_PKG_VERSION")).parse().unwrap());
    headers
}

/// Builds an header that extends the header from `build_http_header`.
/// Adds the api_key to the header.
///
/// # Arguments
/// * `api_key` - The API key to use for the request.
///
/// # Returns
/// A `HeaderMap` containing the headers.
fn build_https_header(api_key: &String) -> HeaderMap {
    let mut headers = build_http_header();
    headers.insert("Authorization: Bearer", api_key.parse().unwrap());
    headers
}
