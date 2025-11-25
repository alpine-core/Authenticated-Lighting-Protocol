# ALPINE — Authenticated Lighting Protocol (v1.0)

ALPINE is a **modern, secure, vendor-agnostic lighting control protocol** designed to replace legacy systems such as sACN/E1.31, RDMnet, and proprietary device APIs.

ALPINE provides:

- **Discovery** — secure device identification without knowing IP
- **Handshake** — mutual authentication & key agreement
- **Control Plane** — reliable, signed envelopes
- **Streaming Layer** — low-latency, real-time lighting frames
- **Capability System** — device declares its features
- **Extensibility** — vendor namespaces, structured envelopes
- **No universes, no DMX limits** — modern frame model

ALPINE is built around:
- **CBOR** for compact structured messages
- **Ed25519** signatures
- **X25519** key exchange
- **UDP broadcast** discovery
- **UDP or QUIC streaming**
- **Deterministic session state machine**

For more details, see the protocol documents:

- [`SPEC.md`](SPEC.md)
- [`docs/architecture.md`](docs/architecture.md)
- [`docs/discovery.md`](docs/discovery.md)
- [`docs/handshake.md`](docs/handshake.md)
- [`docs/control_plane.md`](docs/control_plane.md)
- [`docs/streaming.md`](docs/streaming.md)
- [`docs/capabilities.md`](docs/capabilities.md)
- [`docs/errors.md`](docs/errors.md)
- [`docs/security.md`](docs/security.md)
- [`docs/reference_impl.md`](docs/reference_impl.md)

## Language Bindings

The reference implementation ships with:

- Rust crate (`alpine-core`)
- TypeScript client (`@alpine/protocol`)
- C static library + headers
- Python package (`alpine-protocol`)

Each binding provides:

- Discovery
- Handshake
- Session manager
- Control envelope API
- Streaming client/server

## License

Apache-2.0

