use crate::constants::TEST_IP;
use ip_api4rs::model::ip_response::IpFullResponse;
use ip_api4rs::util::urls::{build_http_url_from_struct, build_https_url_from_struct, build_url_without_fields};

#[test]
fn test_default_http_url() {
    let url = build_url_without_fields(false, &TEST_IP.to_string());
    assert_eq!(url, format!("http://ip-api.com/json/{}", TEST_IP))
}

#[test]
fn test_default_https_url() {
    let url = build_url_without_fields(true, &TEST_IP.to_string());
    assert_eq!(url, format!("https://ip-api.com/json/{}", TEST_IP))
}

#[test]
fn test_custom_http_url() {
    let url = build_http_url_from_struct::<IpFullResponse>(&TEST_IP.to_string());
    assert_eq!(url, format!("http://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,message", TEST_IP));
}

#[test]
fn test_custom_https_url() {
    let url = build_https_url_from_struct::<IpFullResponse>(&TEST_IP.to_string());
    assert_eq!(url, format!("https://ip-api.com/json/{}?fields=query,status,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,reverse,mobile,proxy,hosting,message", TEST_IP));
}
