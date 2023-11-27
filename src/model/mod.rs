//! This module contains all the predefined models for the application.
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
//! #[tokio::main]
//! #[allow(clippy::tabs_in_doc_comments)]
//! async fn main() {
//!    let client = IpApiClient::new();
//!    let result = client.query_api::<Custom>("8.8.8.8").await;
//! }
//! ```

pub mod ip_response;
