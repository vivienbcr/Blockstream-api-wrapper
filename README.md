# **Experimental** Rust wrapper for Blockstream Electrs API

## Description

This library provide a simple wrapper to use Blockstream API or self hosted [Esplora - Electrs API](https://github.com/Blockstream/electrs).
Experimental library

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
....
[dependencies]
esplora-api = { path ="./../Elecrts-wrapper" }
tokio = { version = "0.2", features = ["macros"] }
....
```

```rust
// Main.rs
pub use esplora_api;
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
....
[dependencies]
esplora-api = { path ="./../Elecrts-wrapper", features=["blocking"]  }
....
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
