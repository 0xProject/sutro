# Eye of Sutro: Ethereum State Watcher

<img src="eye_of_sutro.jpg" style="width:25%;float:right;margin:15pt;"></img>

[![Crates.io](https://img.shields.io/crates/l/zkp-stark)](/License.md)
[![Docs.rs](https://docs.rs/zkp-stark/badge.svg)](https://docs.rs/zkp-stark)
[![CircleCI](https://img.shields.io/circleci/build/github/0xProject/OpenZKP)](https://circleci.com/gh/0xProject/OpenZKP)
[![Codecov](https://img.shields.io/codecov/c/gh/0xproject/OpenZKP)](https://codecov.io/gh/0xProject/OpenZKP)

## Idea

Ethereum transactions trigger the execution of EVM contract code. The execution is deterministic and depends only on the transaction (sender, calldata, etc) and the chain state (block info, storage). Transactions can make limited changes to chain state and return a bytestring.

We are not interested in accurately computing gas consumption of transactions or transactions involving creating and destroying contracts. This massively simplifies the EVM semantics. We can also safely ignore logging as it is now redundant.

## Scope

### Order Routing

### Mesh Order Watcher 2.0

### Periscope chain data collector

### Fast test runner

Ganache is quite slow and this prevents us from running as many tests as we would like. A fast EVM engine that can fork of an existing chain.

## Milestones

<https://eth.wiki/json-rpc/API>

* Replay a recent block of transactions.
* Pass all tests in <https://github.com/ethereum/tests>
* Run all solutions from <https://g.solidity.cc/>

Debugging:

* Bytes4 decode any call / return value.
* Parse Solidity sourcemaps.

<br style="clear:both;"/>

```
clear; RUST_LOG="trace,tokio=info,hyper=info,mio=info" cargo run
```
