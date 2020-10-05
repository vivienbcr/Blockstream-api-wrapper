# Rust wrapper for Blockstream Electrs API

## Description

This crate allow you to call Blockstream API or self hosted API easily.
Blockstream explorer API is based on Electrs API. You can run self hosted version of Electrs on you own server for specific usage or / and to improve your privacy.

## Requirements

* [libssl-dev](https://packages.ubuntu.com/fr/xenial/libssl-dev)

```bash
sudo apt install libssl-dev
```

## Dependencies

* Web request framework : [Reqwest](https://docs.rs/reqwest/0.10.8/reqwest/)

## TODO Roadmap :

[X] Implement block

[ ] Implement transactions

[ ] Implement fee-estimates

[ ] Implement mempool

[ ] Implement address

[ ] Upgrade fully customable connection client

[ ] Implement method "as_core" to return bitcoin rust structures for standard usages

[ ] Implement async module and split imports
