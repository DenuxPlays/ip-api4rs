//! A utility module for building requests to the API.

use reqwest::{Client, RequestBuilder};
use serde::Deserialize;

use crate::model::ip_response::IpDefaultResponse;
use crate::util::urls::build_url_from_struct;
use crate::IpApiClient;

/// Builds the request for a default response.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `IpApiClient` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
pub fn build_default_get_request(query: &String, client: &IpApiClient) -> RequestBuilder {
    let url = get_url::<IpDefaultResponse>(query, client);
    match client.api_key {
        Some(_) => build_https_get_request(&url, client.api_key.as_ref().unwrap(), &client.client),
        None => build_http_get_request(&url, &client.client),
    }
}

/// Builds the request for a custom response.
/// The given struct must implement `Deserialize`.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `IpApiClient` to use for the request.
///
/// # Returns
/// A `RequestBuilder` for the request.
pub fn build_get_request<'de, T>(query: &String, client: &IpApiClient) -> RequestBuilder
where
    T: Deserialize<'de>,
{
    let url = get_url::<T>(query, client);
    match client.api_key {
        Some(_) => build_https_get_request(&url, client.api_key.as_ref().unwrap(), &client.client),
        None => build_http_get_request(&url, &client.client),
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
    client.get(url).header("Accept", "application/json").header("Authorization: Bearer", api_key)
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
    client.get(url).header("Accept", "application/json")
}

/// Builds the url for a request.
///
/// # Arguments
/// * `query` - The IP address to query for.
/// * `client` - The `IpApiClient` to use for the request.
///
/// # Returns
/// A `String` containing the url for the request.
fn get_url<'de, T>(query: &String, client: &IpApiClient) -> String
where
    T: Deserialize<'de>,
{
    match client.api_key {
        Some(_) => build_url_from_struct::<T>(true, query),
        None => build_url_from_struct::<T>(false, query),
    }
}
