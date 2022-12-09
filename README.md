![AELO MINING POOL LOG](https://user-images.githubusercontent.com/31732456/202860434-d56edd97-d75a-4dde-a15b-178992a47cb8.png)

# aelo-mining-pool(aleo is deliberately wrong)
[![Rust](https://github.com/harodggg/aelo-mining-pool/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/harodggg/aelo-mining-pool/actions/workflows/rust.yml)
![apache2](https://img.shields.io/hexpm/l/plug?logo=ALEO-MINING-POOL)

## Overview
a mining pool for aleo.The strutum protocol, the mining pool, and the full-node client are independent of each other and separated from each other.

## Features

- pool   (Manage a large number of workers and distribute epoch_challenge to workers)
- worker (Manage prover, the only task is to run a large number of prover)
- client (Connect the prover node to provide the latest block data and epoch_challenge to the worker)


## Getting Started

### Rust Version

`aleo mining pool` currently works on Rust `1.56` and above as it requires support for the 2018 edition.

```bash
rustup update
cargo build
```

### Ubuntu
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev
```
### Mac
```bash
brew install protobuf
```

## Aleo Mining Pool Started

### Aleo Client Usage

### Aleo Pool Usage

### Aleo Worker Usage



## Getting Help
Via github issue is currently the only way to communicate.

## Project Layout
- aleo-pool
- aleo-utils
- aleo-worker
- protos(protobuf files)
- stratum-pool
- stratum-worker

## Contributing
:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][guide] to help you get involved in the Tonic
project.

## License
This project is licensed under the [Apache2 license](LICENSE).

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tonic by you, shall be licensed as Apache2, without any additional
terms or conditions.
