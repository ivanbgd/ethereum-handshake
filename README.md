# Ethereum Handshake Procedure

## Introduction

- This is a client program that dials and connects to an Ethereum node.
- The node's address should be provided by the user through command line.
- TODO: It can also connect to more than one node, successively.
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

## Running

- TODO

## Testing

- TODO

## References

- [Network Addresses](https://ethereum.org/en/developers/docs/networking-layer/network-addresses/)
- [Networking Layer](https://ethereum.org/en/developers/docs/networking-layer/)
- [noise-libp2p - Secure Channel Handshake](https://github.com/libp2p/specs/tree/master/noise)
- [Phase 0 - Networking: The P2P Interface](https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/p2p-interface.md)
- [Recursive-Length Prefix (RLP) Serialization](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/)
- [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
