# Ethereum Handshake Procedure

## Introduction

- This is a client program that dials and connects to an Ethereum node.
- The node's [enode](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/#enode)
  should be provided by the user through command line.
- TODO: It can act as a receiver (a listener), as well,
  not only as an initiator of the connection, making it bidirectional.
- It implements the Ethereum handshake procedure, which is part of the
  Ethereum's [RLPx](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
  transport protocol.  
  We are basically implementing RLPx.
- Peer-to-peer communication is assumed in the Ethereum network.
- Since the purpose of this program is to implement the handshake procedure,
  it doesn't use high-level crates that implement that functionality.
- It instead works at a lower level, and uses lower-level crates.
- Further communication, beyond a successful handshake, is not implemented.
- We are also skipping the node discovery part.

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

## Command Line Arguments

### Arguments

- `<TARGET_ENODE>`: Target node's `enode` in the following form:  
  `enode://<node_id>@<ip_address>:<port>`

### Options

- `-t`, `--timeout <TIMEOUT>`: Handshake timeout in milliseconds, from 100 to 10000 [default: 1000]

## Running

- Substitute `<TARGET_ENODE>` with
  `enode://a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc232016e1c51b544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff05fdf9b31c15072@178.128.136.233:30303`
  in the following examples.
    - This is an Ethereum boot node running on the Holesky test network.
    - Boot node addresses can be found at:  
      https://github.com/ethereum/go-ethereum/blob/master/params/bootnodes.go
- Alternatively, pick a node from:
    - https://etherscan.io/nodetracker/nodes
    - https://ethernodes.org/nodes
    - Make sure to grab the full
      [enode](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/#enode).
- Executables can be downloaded from the repository's
  [Releases](https://github.com/ivanbgd/ethereum-handshake/releases) page.
    - `ethereum-handshake [-t <TIMEOUT>] <TARGET_ENODE>`
        - Example: `ethereum-handshake -t 800 <TARGET_ENODE>`
- Alternatively, by using `cargo`:
    - `cargo run [--release] -- [-t <TIMEOUT>] <TARGET_ENODE>`
        - Example: `cargo run -- -t 2000 <TARGET_ENODE>`

## Testing

- TODO

## References

- [Network Addresses](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/)
- [Networking Layer](https://ethereum.org/en/developers/docs/networking-layer/)
- [noise-libp2p - Secure Channel Handshake](https://github.com/libp2p/specs/tree/master/noise)
- [Phase 0 - Networking: The P2P Interface](https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/p2p-interface.md)
- [Recursive-Length Prefix (RLP) Serialization](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/)
- [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)