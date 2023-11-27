use crate::constants::EXTERN_TEST_IP;
use ip_api4rs::model::ip_response::IpDefaultResponse;

pub fn get_expected_default_response() -> IpDefaultResponse {
    IpDefaultResponse {
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
        asn: "AS15169 Google LLC".to_string(),
    }
}
