# **Unofficial** Rust wrapper for Blockstream Electrs API

## Description

This library provide a simple wrapper to use Blockstream API or self hosted [Esplora - Electrs API](https://github.com/Blockstream/electrs). You can run self hosted version of Electrs on you own server for specific usages or/and to improve your privacy.

## Requirements

* Reqwest framework require [libssl-dev](https://packages.ubuntu.com/fr/xenial/libssl-dev)

```bash
sudo apt install libssl-dev
```

## Dependencies

* Web request framework : [Reqwest](https://docs.rs/reqwest/0.10.8/reqwest/)

* Serde deserialize : [Serde](https://crates.io/crates/serde)

## Roadmap

* Upgrade fully customable connection client

* Implement method "as_core" to return bitcoin rust structures for standard usages

* Implement Liquid routes
