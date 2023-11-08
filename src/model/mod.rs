//! Contains all the predefined models for the application.
//! You can create your own models to customize the request.
//!
//! # Supplying your own struct - example
//! ```
//! use serde::{Deserialize, Serialize};
//! use ip_api4rs::IpApiClient;
//!
//! #[derive(Deserialize, Serialize)]
//! struct Custom {
//!    #[serde(rename = "query")]
//!    ip: String,
//!    country: String,
//! }
//!
//!    let client = IpApiClient::new();
//!    let result = client.query_api::<Custom>("8.8.8.8".to_string()).await;
//! ```

pub mod ip_response;
