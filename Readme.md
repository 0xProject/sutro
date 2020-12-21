# Eye of Sutro: Ethereum State Watcher

![lines of code](https://img.shields.io/tokei/lines/github/0xProject/sutro)
[![docs.rs](https://docs.rs/sutro/badge.svg)](https://docs.rs/sutro)
[![dependency status](https://deps.rs/repo/github/0xProject/sutro/status.svg)](https://deps.rs/repo/github/0xProject/sutro)
[![codecov](https://img.shields.io/codecov/c/github/0xProject/sutro)](https://codecov.io/gh/0xProject/sutro)
[![build](https://img.shields.io/github/workflow/status/0xProject/sutro/build)](https://github.com/0xProject/sutro/actions?query=workflow%3Abuild)

<img src="eye_of_sutro.jpg" width="33%" align="right" style="padding-left: 20px"></img>

**Idea**

Ethereum transactions trigger the execution of EVM contract code. The execution is deterministic and depends only on the transaction (sender, calldata, etc) and the chain state (block info, storage). Transactions can make limited changes to chain state and return a bytestring.

We are not interested in accurately computing gas consumption of transactions or transactions involving creating and destroying contracts. This massively simplifies the EVM semantics. We can also safely ignore logging as it is now redundant.

<br style="clear:both;"/>

## Scope

### Order Routing

### Mesh Order Watcher 2.0

### Periscope chain data collector

### Fast test runner

Ganache is quite slow and this prevents us from running as many tests as we would like. A fast EVM engine that can fork of an existing chain.

## Milestones

<https://eth.wiki/json-rpc/API>

* Forward RPC to Ganache and pass tests.
* Replay a recent block of transactions.
* Replace <https://github.com/0xProject/go-ethereum>
* Pass all tests in <https://github.com/ethereum/tests>
* Run all solutions from <https://g.solidity.cc/>

Debugging:

* Bytes4 decode any call / return value.
* Parse Solidity sourcemaps.

```
clear; RUST_LOG="trace,tokio=info,hyper=info,mio=info" cargo run
```
