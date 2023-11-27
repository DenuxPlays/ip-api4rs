# ip-api4rs &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs]][docs.rs]

[Build Status]: https://img.shields.io/github/actions/workflow/status/DenuxPlays/ip-api4rs/rust.yml?branch=main&style=flat-square
[actions]: https://github.com/DenuxPlays/ip-api4rs/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/ip-api4rs.svg?style=flat-square
[crates.io]: https://crates.io/crates/ip-api4rs
[docs.rs]: https://docs.rs/ip-api4rs
[Docs]: https://img.shields.io/badge/docs.rs-ip--api4rs-66c2a5?style=flat-square&color=blue

A simple but powerful wrapper for the [ip-api](https://ip-api.com/) API.

## Usage

In the example below we use tokio to convert our async main function into a sync one.<br>
Tokio itself is not needed when using this crate.
```rust
use ip_api4rs::IpApiClient;

#[tokio::main]
async fn main() {
	let client = IpApiClient::new();
	let response = client.query_api_default("8.8.8.8").await.unwrap();
	println!("{:#?}", response);
}
```


## Features

- fully async api (or blocking with the `blocking` feature)
- simple to use
- supply custom structs to only get want you want
- Api-Token support
- SSL support

## Features that won't be added
**This doesn't mean that we would not implement features listed here.**<br>
We just won't implement them ourselves.<br>
Feel free to open a pull request if you want to add more features.

- Batch requests
- non Json responses
- DNS Api support
