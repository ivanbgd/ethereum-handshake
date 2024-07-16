# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Acting as a recipient (responding to a handshake request)
- Connecting to and/or from multiple nodes at the same time

## [0.1.0] - 2024-07-16 - TODO

This is the very first (initial) fully-functioning version of the library and the program.

The program acts as a client and calls another node.

The handshake procedure is implemented in that case.

### Added

- Library crate:
    - The main business logic function (public),
    - Helper functions (private).
- Binary (executable) crate, which uses the library.
- `README.md`
- `LICENSE` ("MIT")
- `CHANGELOG.md`
- GitHub actions: `audit.yml`, `ci.yml`, `release.yml`
- Pre-push hooks
