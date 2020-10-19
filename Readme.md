# Eye of Sutro: Ethereum State Watcher

<img src="eye_of_sutro.jpg" style="width:25%;float:right;"></img>

[![Crates.io](https://img.shields.io/crates/l/zkp-stark)](/License.md)
[![Docs.rs](https://docs.rs/zkp-stark/badge.svg)](https://docs.rs/zkp-stark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/OpenZKP)](https://circleci.com/gh/0xProject/OpenZKP)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/OpenZKP)](https://codecov.io/gh/0xProject/OpenZKP)

<br style="clear:both;"/>

## Milestones

<https://eth.wiki/json-rpc/API>

* Replay a recent block of transactions.
* Pass all tests in <https://github.com/ethereum/tests>
* Run all solutions from <https://g.solidity.cc/>

Debugging:

* Bytes4 decode any call / return value.
* Parse Solidity sourcemaps.

```
clear; RUST_LOG="trace,tokio=info,hyper=info,mio=info" cargo run
```
