use serde::de::DeserializeOwned;
use tokio::runtime::Runtime;

use crate::error::IpApiError;
use crate::model::ip_response::{IpDefaultResponse, IpFullResponse};
use crate::IpApiClient;

/// A client for the ip-api.com API that blocks the current thread.
struct BlockingIpApiClient {
    /// The inner client to use for the requests.
    pub inner_client: IpApiClient,
    /// The runtime to use for the requests.
    pub runtime: Runtime,
}

impl Default for BlockingIpApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockingIpApiClient {
    /// Creates a new IpApiClient with no API key.
    pub fn new() -> Self {
        Self {
            inner_client: IpApiClient::new(),
            runtime: Self::build_runtime(),
        }
    }

    /// Creates a new IpApiClient with an API key.
    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            inner_client: IpApiClient::new_with_api_key(api_key),
            runtime: Self::build_runtime(),
        }
    }

    /// Builds the tokio runtime that will be used for the blocking requests.
    fn build_runtime() -> Runtime {
        return tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    }

    /// Queries the API with the default fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpDefaultResponse` - The response from the API.
    pub fn query_api_default(&self, ip: &String) -> Result<IpDefaultResponse, IpApiError> {
        return self.runtime.block_on(self.inner_client.query_api_default(ip));
    }

    /// Queries the API with all fields.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `IpFullResponse` - The response from the API.
    pub fn query_api_fully(&self, ip: &String) -> Result<IpFullResponse, IpApiError> {
        return self.runtime.block_on(self.inner_client.query_api_fully(ip));
    }

    /// Queries the API with a custom struct.
    ///
    /// # Arguments
    /// * `ip` - The IP address to query.
    ///
    /// # Returns
    /// * `T` - The response from the API.
    pub async fn query_api<T>(&self, ip: &String) -> Result<T, IpApiError>
    where
        T: DeserializeOwned,
    {
        return self.runtime.block_on(self.inner_client.query_api::<T>(ip));
    }
}
