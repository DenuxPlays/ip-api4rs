//! A module that helps you building the correct url for a request.

use serde::Deserialize;
use serde_aux::prelude::serde_introspect;

/// Builds the url for a request.
/// The fields are automatically extracted from the struct.
///
/// # Arguments
/// * `https` - A `bool` indicating whether the url should be built for a HTTPS request or not.
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the url for the request.
pub fn build_url_from_struct<'de, T>(https: bool, ip: &String) -> String
where
    T: Deserialize<'de>,
{
    let fields = serde_introspect::<T>();
    build_url_with_fields(https, ip, fields)
}

/// Builds the HTTP url for a request.
/// The fields are automatically extracted from the struct.
///
/// # Arguments
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the url for the request.
pub fn build_http_url_from_struct<'de, T>(ip: &String) -> String
where
    T: Deserialize<'de>,
{
    let fields = serde_introspect::<T>();
    build_url_with_fields(false, ip, fields)
}

/// Builds the HTTPS url for a request.
/// The fields are automatically extracted from the struct.
///
/// # Arguments
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the url for the request.
pub fn build_https_url_from_struct<'de, T>(ip: &String) -> String
where
    T: Deserialize<'de>,
{
    let fields = serde_introspect::<T>();
    build_url_with_fields(true, ip, fields)
}

/// Builds the url for a request without fields.
/// # Arguments
/// * `https` - A `bool` indicating whether the url should be built for a HTTPS request or not.
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the url for the request.
pub fn build_url_without_fields(https: bool, ip: &String) -> String {
    match https {
        true => format!("{}{}", crate::constant::HTTPS_HOST, ip),
        false => format!("{}{}", crate::constant::HTTP_HOST, ip),
    }
}

/// Builds the url for a request with fields.
/// # Arguments
/// * `https` - A `bool` indicating whether the url should be built for a HTTPS request or not.
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the url for the request.
pub fn build_url_with_fields(https: bool, ip: &String, fields: &[&'static str]) -> String {
    let mut fields_str = fields.join(",");
    if !fields.contains(&"message") {
        //We do this to get the error message if an error occurs.
        fields_str = fields.join(",") + ",message";
    }
    match https {
        true => format!("{}{}?fields={}", crate::constant::HTTPS_HOST, ip, fields_str),
        false => format!("{}{}?fields={}", crate::constant::HTTP_HOST, ip, fields_str),
    }
}
