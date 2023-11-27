use crate::constants::EXTERN_TEST_IP;
use crate::end2end::client::utils::get_expected_default_response;
use ip_api4rs::client::AsyncIpApi;
use ip_api4rs::error::IpApiError;
use ip_api4rs::model::ip_response::IpDefaultResponse;
use ip_api4rs::IpApiClient;

#[tokio::test]
async fn test_client() {
    let client = IpApiClient::new();
    assert_eq!(client.api_key, None);
    let result = client.query_api_default(EXTERN_TEST_IP).await;
    let expected = get_expected_default_response();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[tokio::test]
async fn test_client_custom() {
    let client = IpApiClient::new();
    assert_eq!(client.api_key, None);
    let result = client.query_api::<IpDefaultResponse>(EXTERN_TEST_IP).await;
    let expected = get_expected_default_response();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[tokio::test]
async fn test_error_reserved_range() {
    let client = IpApiClient::new();
    let result = client.query_api::<IpDefaultResponse>("127.0.0.1").await;
    match result.err().unwrap() {
        IpApiError::ReservedRange(error_response) => {
            assert_eq!(error_response.message, "reserved range");
        }
        _ => panic!("Wrong error type returned."),
    }
}

#[tokio::test]
async fn test_error_invalid_query() {
    let client = IpApiClient::new();
    let result = client.query_api::<IpDefaultResponse>("Invalid Query").await;
    match result.err().unwrap() {
        IpApiError::InvalidQuery(error_response) => {
            assert_eq!(error_response.message, "invalid query");
        }
        _ => panic!("Wrong error type returned."),
    }
}
