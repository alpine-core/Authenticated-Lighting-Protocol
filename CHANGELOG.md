# Changelog

All notable changes to ALPINE will be documented in this file.

## [1.0.2] - 2025-11-27
- Align TS/GitHub package workflows with the `@alpine-core` scope and add npmjs/public flags.
- Fix release artifacts to copy the actual crate/static lib names and expose Python wheels.
- Tag the repo `v1.0.2` so CI can publish all bindings again.

## [1.0.0] - 2025-11-23
- First public release of ALPINE v1.
- Deterministic session state machine and authenticated control plane over UDP.
- X25519 + Ed25519 security model with signed control envelopes.
- Reliable control channel (retransmit/backoff/replay protection).
- ALNP-Stream gating with jitter handling (hold-last, drop, lerp).
- TypeScript and C bindings scaffolds; Python package stub for clients.

[1.0.2]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.2
[1.0.0]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.0
