use serde::{Deserialize, Serialize};

/// A struct containing the complete response from the Ip Api.
/// We ask the Api to return every field it can.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct IpFullResponse {
    pub query: String,
    pub status: String,
    pub continent: String,
    #[serde(rename = "continentCode")]
    pub continent_code: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub region: String,
    #[serde(rename = "regionName")]
    pub region_name: String,
    pub city: String,
    pub district: String,
    pub zip: String,
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub offset: i32,
    pub currency: String,
    pub isp: String,
    pub org: String,
    #[serde(rename = "as")]
    pub as_number: String,
    #[serde(rename = "asname")]
    pub as_name: String,
    pub reverse: String,
    pub mobile: bool,
    pub proxy: bool,
    pub hosting: bool,
}

/// A struct containing the default response from the Ip Api.
/// We simply give the Api the Ip address with no extra parameters.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct IpDefaultResponse {
    pub query: String,
    pub status: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub region: String,
    #[serde(rename = "regionName")]
    pub region_name: String,
    pub city: String,
    pub zip: String,
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub isp: String,
    pub org: String,
    #[serde(rename = "as")]
    pub as_number: String,
}

/// A module that contains the error type for the library.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub query: String,
}
