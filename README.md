# Ethereum Handshake Procedure

[![CI](https://github.com/ivanbgd/ethereum-handshake/actions/workflows/ci.yml/badge.svg)](https://github.com/ivanbgd/ethereum-handshake/actions/workflows/ci.yml)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)

## Introduction

- This is a client program that dials and connects to an Ethereum node.
- The node's [enode](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/#enode)
  can be provided by the user through command line and/or through a text file
  which consists of a list of enodes, with an enode per line of the text file.
    - It should contain an IPv4 address, which is almost always the case with
      Ethereum nodes.
    - All valid enodes from command line and the file are included.
    - Invalid enodes are simply skipped.
- TODO: It can act as a receiver (a listener), as well,
  not only as an initiator of the connection, making it bidirectional.
- It implements the Ethereum handshake procedure, which is part of the
  Ethereum's [RLPx](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
  transport protocol.
    - We are basically implementing RLPx.
- Peer-to-peer communication is assumed in the Ethereum network.
- Since the purpose of this program is to implement the handshake procedure,
  it doesn't use high-level crates that implement that functionality.
- It instead works at a lower level, and uses lower-level crates.
- Further communication, beyond a successful handshake, is not implemented.
- We are also skipping the node discovery part.
- We provide a configurable timeout for establishing a TCP connection,
  and for completing a full handshake procedure.

## Implementation

- This program doesn't use high-level libraries such as
  [libp2p](https://libp2p.io/) or [devp2p](https://github.com/ethereum/devp2p).
- It uses cryptographic libraries,
  which are necessary for the handshake procedure.
- It also uses a library for
  [RLP](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/),
  which is Ethereum's serialization algorithm.
- This project uses asynchrony for concurrent execution. We use the
  [tokio](https://crates.io/crates/tokio) library for that.
- We keep the main function minimal.
- In case we add support for receiving calls and/or for supporting multiple
  connections at once, the design/architecture of the project will probably
  change, at least a little.
    - The caveat is that each handshake should preferably complete as an atomic
      operation, meaning it shouldn't be interrupted until it's complete.
- Input validation is performed at a single place, which is at the program's
  boundary at which the data (which is recipient's enode's) enters the application.

## Development

### Pre-Push Checks

Pre-push hooks are available in this repository.

A Python package is required to use them. Install it by executing:

`pip3 install pre-commit`

Inside the repository, run:

`pre-commit install --hook-type pre-push`

The hooks will be run automatically before each push.

We can also run them manually like this:

`pre-commit run --all-files`

## Command Line Arguments

### Arguments

There are no required arguments.

TODO Level 1: If recipient isn't provided, the application will act only as a receiver.
TODO Level 2: The application is bidirectional.

### Options

- `-t`, `--timeout <TIMEOUT>`: Handshake timeout in milliseconds, from 100 to 10000 [default: 1000]
- `-r`, `--recipient-enode <RECIPIENT_ENODE>`: Recipient node's `enode` in the following form:  
  `enode://<node_id>@<ipv4_address>:<port>`
    - This is a list of `enode`s, so there can be more than one; just prepend
      each with `-r`.
- `-f`, `--file-path <FILE_PATH>`: Path to a text file with a list of
  recipient `enode`s in the following form:  
  `enode://<node_id>@<ipv4_address>:<port>`

## Running

- You can optionally set the `RUST_LOG` environment variable to `debug` or `trace` to
  have more detailed log messaging.
    - The log level is set to `info` by default.
    - `export RUST_LOG=debug`
    - `export RUST_LOG=trace`
- In the following examples, substitute `<RECIPIENT_ENODE>` with
  `enode://a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc232016e1c51b544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff05fdf9b31c15072@178.128.136.233:30303`
    - This is an Ethereum boot node running on the Holesky test network.
    - Boot node addresses can be found at:  
      https://github.com/ethereum/go-ethereum/blob/master/params/bootnodes.go
    - Boot nodes should be up and running all the time.
- Alternatively, pick a node from:
    - https://etherscan.io/nodetracker/nodes
    - https://ethernodes.org/nodes
    - Make sure to grab a full
      [enode](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/#enode).
- Executables can be downloaded from the repository's
  [Releases](https://github.com/ivanbgd/ethereum-handshake/releases) page.
    - `ethereum-handshake [-t <TIMEOUT>] [-r <RECIPIENT_ENODE>] [-f FILE_PATH]`
        - Example: `ethereum-handshake -t 2500 -r <RECIPIENT_ENODE> -f <FILE_PATH> -r <RECIPIENT_ENODE>`
- Alternatively, by using `cargo`:
    - `cargo run [--release] -- [-t <TIMEOUT>] [-r <RECIPIENT_ENODE>]`
        - Example: `cargo run -- -r <RECIPIENT_ENODE>`
        - Example: `cargo run -- -f <FILE_PATH>`

## Testing

- Unit tests are implemented.
    - They can be performed by executing:  
      `cargo test`
- Manual testing can be performed by running the application from the CLI,
  as described above.

## References

- [Networking Layer](https://ethereum.org/en/developers/docs/networking-layer/)
    - [Network Addresses](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/)
- [Phase 0 - Networking: The P2P Interface](https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/p2p-interface.md)
- [Recursive-Length Prefix (RLP) Serialization](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/)
- [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
