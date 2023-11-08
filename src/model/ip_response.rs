use serde::{Deserialize, Serialize};

/// A struct containing the complete response from the Ip Api.
/// We ask the Api to return every field it can.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct IpFullResponse {
    query: String,
    status: String,
    continent: String,
    #[serde(rename = "continentCode")]
    continent_code: String,
    country: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    region: String,
    #[serde(rename = "regionName")]
    region_name: String,
    city: String,
    district: String,
    zip: String,
    lat: f32,
    lon: f32,
    timezone: String,
    offset: i32,
    currency: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    as_number: String,
    #[serde(rename = "asname")]
    as_name: String,
    reverse: String,
    mobile: bool,
    proxy: bool,
    hosting: bool,
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
