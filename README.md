# Rust wrapper for Blockstream Electrs API

## Description

This library provide wrapper to use Blockstream API or self hosted [Esplora - Electrs API](https://github.com/Blockstream/electrs).
Blockstream explorer API is based on Electrs API. You can run self hosted version of Electrs on you own server for specific usage or / and to improve your privacy.

## Requirements

* [libssl-dev](https://packages.ubuntu.com/fr/xenial/libssl-dev)

```bash
sudo apt install libssl-dev
```

## Dependencies

* Web request framework : [Reqwest](https://docs.rs/reqwest/0.10.8/reqwest/)

* Serde : [Bro in the storm](https://crates.io/crates/serde)

## Roadmap

* Upgrade fully customable connection client

* Implement method "as_core" to return bitcoin rust structures for standard usages

* Implement async module and split imports (cf reqwest crate)

* Implement Liquid routes
