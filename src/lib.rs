//! # Blockstream Api Wrapper
//! 
//! This library provide a simple wrapper to use Blockstream API or self hosted [Esplora - Electrs API](https://github.com/Blockstream/electrs) based on reqwest framework.
//! Wrapper can be used with custom configuration according to your needs.
//! Liquid features not implemented for the moment.
//! 
//! ## Optionnal Features 
//! - **blocking**: Provides the [blocking](blocking) client API.
//! 
//! ## Usage
//! 
//! Simple async usage : 
//! 
//! ````rust
//! fn main(){
//!    let client = esplora_api::blocking::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
//!    let res = client.get_address("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk").unwrap();
//!    println!("{:?}",res);
//! }
//! ````
//! 
//! Custom reqwest client:
//! 
//! ````rust
//! use reqwest;
//! use reqwest::header;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut headers = header::HeaderMap::new();
//!    headers.insert(header::AUTHORIZATION,header::HeaderValue::from_static("secret"));
//!    let reqwest_client = reqwest::Client::builder().default_headers(headers).build()?;
//!    let client = esplora_api::async_impl::ApiClient::new_from_config("https://blockstream.info/testnet/api/",reqwest_client).unwrap();
//!    let response = client.get_address("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk").await?;
//!    println!("{:?}", response);
//!   Ok(())
//! }
//!````
//! 
//! 
//! 
pub mod async_impl;
pub mod data;

#[cfg(feature = "blocking")]
pub mod blocking;
