# Rust wrapper for Blockstream Esplora API

[![Crates.io](https://img.shields.io/crates/v/esplora-api)](https://crates.io/crates/esplora-api) [![DocRs](https://docs.rs/esplora-api/badge.svg?version=0.1.0)](https://docs.rs/esplora-api/0.1.0/esplora_api/) [![codecov](https://codecov.io/gh/vivienbcr/Blockstream-api-wrapper/branch/master/graph/badge.svg?token=7P0MURXOOO)](undefined)
[![Actions Status](https://github.com/vivienbcr/Blockstream-api-wrapper/workflows/ci/badge.svg)](https://github.com/vivienbcr/Blockstream-api-wrapper/actions) 

## Description

This library provide a simple wrapper to use Blockstream API or self hosted [Esplora - Electrs API](https://github.com/Blockstream/electrs).

## Requirements

* Reqwest framework require [libssl-dev](https://packages.ubuntu.com/fr/xenial/libssl-dev)

```bash
sudo apt install libssl-dev
```

## Dependencies

* Web request framework : [Reqwest](https://docs.rs/reqwest/0.10.8/reqwest/)

* Serde deserialize : [Serde](https://crates.io/crates/serde)

## Use

### Async implementation

```toml
// Cargo.toml
[dependencies]
esplora-api = { path ="./../Elecrts-wrapper" }
tokio = { version = "0.2", features = ["macros"] }
```

```rust
// Main.rs
use esplora_api;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = esplora_api::async_impl::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    let res = client.get_address("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk").await?;
    println!("{:?}",res);
    Ok(())
}
```

### Blocking implementation

```toml
// Cargo.toml
[dependencies]
esplora-api = { path ="./../Elecrts-wrapper", features=["blocking"]  }
```

```rust
// Main.rs
pub use esplora_api;
fn main(){
    let client = esplora_api::blocking::client::ApiClient::new("https://blockstream.info/testnet/api/", None).unwrap();
    let res = client.get_address("n1vgV8XmoggmRXzW3hGD8ZNTAgvhcwT4Gk").unwrap();
    println!("{:?}",res);
}
```
