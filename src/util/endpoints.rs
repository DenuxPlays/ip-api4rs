//! A module that helps you building the correct endpoint for a request.

/// Builds the endpoint for a HTTP request.
/// # Arguments
/// * `ip` - The IP address to query for.
/// * `fields` - The fields to query for. If `None` is passed, the endpoint will be built without fields.
///
/// # Returns
/// A `String` containing the endpoint url (http) for the request.
pub fn build_http_endpoint(ip: &String, fields: Option<&[String]>) -> String {
    match fields {
        Some(fields) => build_endpoint_with_fields(false, ip, fields),
        None => build_endpoint_without_fields(false, ip),
    }
}

/// Builds the endpoint for a HTTPS request.
/// # Arguments
/// * `ip` - The IP address to query for.
/// * `fields` - The fields to query for. If `None` is passed, the endpoint will be built without fields.
///
/// # Returns
/// A `String` containing the endpoint url (https) for the request.
pub fn build_https_endpoint(ip: &String, fields: Option<&[String]>) -> String {
    match fields {
        Some(fields) => build_endpoint_with_fields(true, ip, fields),
        None => build_endpoint_without_fields(true, ip),
    }
}

/// Builds the endpoint for a request without fields.
/// # Arguments
/// * `https` - A `bool` indicating whether the endpoint should be built for a HTTPS request or not.
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the endpoint url for the request.
fn build_endpoint_without_fields(https: bool, ip: &String) -> String {
    match https {
        true => format!("{}/{}", crate::constant::HTTPS_HOST, ip),
        false => format!("{}/{}", crate::constant::HTTP_HOST, ip),
    }
}

/// Builds the endpoint for a request with fields.
/// # Arguments
/// * `https` - A `bool` indicating whether the endpoint should be built for a HTTPS request or not.
/// * `ip` - The IP address to query for.
///
/// # Returns
/// A `String` containing the endpoint url for the request.
fn build_endpoint_with_fields(https: bool, ip: &String, fields: &[String]) -> String {
    match https {
        true => format!("{}/{}?fields={}", crate::constant::HTTPS_HOST, ip, fields.join(",")),
        false => format!("{}/{}?fields={}", crate::constant::HTTP_HOST, ip, fields.join(",")),
    }
}
