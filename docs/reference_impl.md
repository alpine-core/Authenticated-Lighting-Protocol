# ALPINE Reference Implementation Guide

This document describes how to implement ALPINE 1.0 in:

- Rust (canonical implementation)
- TypeScript
- Python
- C (static library)
- C++ helper header + `ALPINE_EMBEDDED` profile
- Language-specific SDK helpers (Rust `sdk`, TypeScript `sdk`, Python `sdk`, C++ `sdk`)
- C++ (helper header wrapping the C helpers)

A correct implementation MUST:

1. Implement CBOR encoding/decoding
2. Support UDP broadcast discovery
3. Implement handshake state machine
4. Maintain session state with expiry
5. Support control envelopes
6. Support streaming envelopes
7. Validate signatures and MAC tags
8. Handle capability negotiation
9. Follow error semantics exactly

Reference code structure is included for each language; C++ users can toggle
`ALPINE_EMBEDDED` to compile the same header without heap allocations or RTTI.
