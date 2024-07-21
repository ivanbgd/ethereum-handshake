# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Acting as a recipient (responding to a handshake request)
- Connecting to and/or from multiple nodes at the same time (atomically)

## [0.1.0] - 2024-07-?? - WIP, TODO

This is the very first (initial) fully-functioning version of the library and the program.

The program supports the initiator mode, acting as a client and calling another node(s).

Recipient's enodes can be provided via CLI and/or a text file.

The handshake procedure is implemented for the initiator mode.

### Added

- Library crate
- Binary (executable) crate, which uses the library
- `README.md`
- `LICENSE` ("MIT")
- `CHANGELOG.md`
- GitHub actions: `audit.yml`, `ci.yml`, `release.yml`
- Pre-push hooks
