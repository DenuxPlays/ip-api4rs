//! # Constants
//!
//! This module defines the constants used in the library

/// Defines the http host for the ip-api.com service
pub const HTTP_HOST: &str = "http://ip-api.com/json/";

/// Defines the https host for the ip-api.com service
pub const HTTPS_HOST: &str = "https://ip-api.com/json/";

/// The string to check the message against when an error occurred.
pub const ERROR_RESERVED_RANGE: &str = "reserved range";

/// The string to check the message against when an invalid query occurred.
pub const ERROR_INVALID_QUERY: &str = "invalid query";
