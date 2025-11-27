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

## Continuous Integration

- `UDP E2E Tests` workflow (`.github/workflows/e2e-tests.yml`) runs `cargo test --tests -- --ignored` from `src/alnp`, exercising the real UDP handshake/control/streaming paths on Linux.

## Publishing & package registries

This project publishes artifacts for Rust, C, TypeScript, and Python. Before running the release scripts or invoking `cargo publish`, set the following credentials in your environment or GitHub secrets:

| Registry | Environment variable | Notes |
| --- | --- | --- |
| GitHub Packages (Rust) | `CARGO_REGISTRIES_GITHUB_TOKEN` | Used by `cargo publish --registry github`. The GitHub registry index is configured in `src/alnp/Cargo.toml`; also point `CARGO_HOME` at `<repo>/.cargo` (or add the same `[registries.github]` entry inside your global Cargo config) so that the registry is loaded before publishing—otherwise `cargo publish --registry github` will panic with “remote registries must have config”. |
| TypeScript (npm/PNPM) | `NPM_TOKEN` or `PNPM_TOKEN` | Required for publishing `dist/ts` via `npm publish` / `pnpm publish`. |
| Python (PyPI or GitHub) | `PYPI_API_TOKEN` (or `TWINE_USERNAME`/`TWINE_PASSWORD`) | `scripts/build_python.sh` generates wheel/sdist artifcats; upload them with `twine upload`. |
| C artifacts | `GITHUB_TOKEN` | Use this token to push `dist/c` (static library + header) to GitHub Packages or release assets. |

## Language Bindings

The reference implementation ships with:

- Rust crate (`alpine-protocol-rs`)
- TypeScript client (`@alpine-core/protocol`)
- C static library + headers
- C++ helper header (`bindings/cpp/alnp.hpp`)
- Python package (`alpine-protocol`)

Each binding provides:

- Discovery
- Handshake
- Session manager
- Control envelope API
- Streaming client/server

## SDK layers

- **Rust**: `src/alnp/src/sdk` exposes `AlpineClient`, which orchestrates discovery, handshake, streaming frames, and keepalive over UDP while reusing the control/crypto helpers.
- **TypeScript**: `bindings/ts/src/sdk/client.ts` wraps the binding helpers with a Node UDP client that exposes `discover()`, `handshake()`, and `sendFrame()` convenience methods.
- **Python**: `bindings/python/src/alnp/sdk` provides a socket-driven class that builds CBOR discovery/control/frame payloads and leaves network I/O to the consumer.
- **C++**: `bindings/cpp/sdk/alpine_sdk.hpp` defines `AlpineTransport` and `AlpineClient` so you can feed encoded discovery/control/frame bytes into your own transport implementation.

Use these SDKs as the primary application entry points, and reserve the auto-generated low-level bindings for embedded or constrained contexts.

## Embedded mode

The C++ helper exposes an `ALPINE_EMBEDDED` configuration that keeps the API
identical while disabling hidden heap allocations, RTTI, and exception support.
Use `scripts/build_embedded_cpp.sh` or `bindings/cpp/embedded_test.cpp` to verify
the restricted flags (`-fno-exceptions`, `-fno-rtti`, `-fno-threadsafe-statics`,
`-fno-use-cxa-atexit`, `-Os`). The CI job `.github/workflows/embedded.yml` runs
that script for every change to `main`, so ESP32-style builds are validated
alongside the desktop releases.

## License

Apache-2.0
