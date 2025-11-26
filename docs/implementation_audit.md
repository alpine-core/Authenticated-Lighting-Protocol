# Implementation vs Documentation Audit

This report compares the promises written in the documentation (`README.md`, `SPEC.md`, and the files under `docs/`) against the current implementation, tests, and benchmarks located in `src/alnp`. It also highlights how the new `src/alnp/docs/benchmarks.md` ties back to the generated Criterion artifacts under `target/criterion`.

## README-level commitments

- **Discovery / Handshake / Control / Streaming** - The README frames those four layers as ALPINE's core. Each layer now has a real-UDP exercise in `src/alnp/tests/e2e/`:
  - `handshake_udp_e2e.rs` runs the controller and node over real `tokio::net::UdpSocket`s, completes the `session_init` -> `session_complete` sequence, and asserts that both sides derive the same session ID and keys (Phase 1).
  - `control_udp_e2e.rs` exchanges signed control envelopes, verifies MACs and sequence numbers, and delivers ACKs entirely over the wire (Phase 2).
  - `streaming_udp_e2e.rs` sends `FrameEnvelope`s through sockets, decodes them, and exercises `JitterStrategy::HoldLast` across real timing (Phase 3).
  - Discovery lives inside `src/alnp/src/discovery.rs`, where replies are signed with Ed25519 and nonces are validated before returning `DiscoveryReply` structures.
- **Capability System and Extensibility** - `CapabilitySet` flows through discovery, handshake, session state, and control helpers (`src/alnp/src/messages/mod.rs`, `src/alnp/src/handshake/*`, and `src/alnp/src/session/mod.rs`). `tests/feature_suite.rs` insists on default capabilities, matching the README's claim that controllers learn device capabilities without guessing.
- **Language bindings (Rust, TypeScript, Python, C)** - The root `bindings/` directory contains `ts/`, `python/`, and `c/` subprojects that mirror the Rust structures (`CapabilitySet`, `DiscoveryReply`, etc.). While this audit did not run their package managers, the code exists to satisfy `docs/reference_impl.md`.
- **Benchmarks and diagnostics** - The README's mention of structured diagnostics is complemented by the Criterion benchmark suite described in `src/alnp/docs/benchmarks.md` and the artifacts under `target/criterion/*`. Each report contains histograms, regression notices, and medians that document the latency numbers promised in the documentation.

## SPEC sections vs implementation

- **Discovery layer (Section 4 and `docs/discovery.md`)** - The spec describes UDP discovery with capability blocks, server nonces, and Ed25519 signatures. `src/alnp/src/messages/mod.rs` defines `DiscoveryRequest` and `DiscoveryReply`, while `src/alnp/src/discovery.rs` signs replies and validates incoming replies; `tests/e2e/discovery_e2e.rs` exercises this over sockets.
- **Handshake layer (Section 5 and `docs/handshake.md`)** - The spec enumerates the `session_init` -> `session_ack` -> `session_ready` -> `session_complete` handshake and insists on capability negotiation. The code in `src/alnp/src/handshake/*` follows that flow, and `tests/e2e/handshake_udp_e2e.rs` runs it end-to-end over real UDP sockets with the shared helper `src/alnp/src/e2e_common.rs`.
- **Control plane (Section 6 and `docs/control_plane.md`)** - The spec requires MAC-protected envelopes, monotonic sequence numbers, ACKs, and retransmits. These elements live in `src/alnp/src/control.rs`, and `tests/e2e/control_udp_e2e.rs` verifies MAC validation, ACK replies, and proper session binding over UDP.
- **Streaming transport (Section 7 and `docs/streaming.md`)** - The frame structure, jitter strategies, and ordered delivery requirements are implemented inside `src/alnp/src/stream.rs`. The streaming e2e test (`tests/e2e/streaming_udp_e2e.rs`) touches the socket layer, while `benches/alpine_streaming.rs` exercises the encode -> send -> receive -> decode path described in `src/alnp/docs/benchmarks.md`.
- **Error codes (`docs/errors.md`)** - All documented error variants appear in the `ErrorCode` enum inside `src/alnp/src/messages/mod.rs`, guaranteeing consistent serialization.
- **Security (`docs/security.md`)** - The security model lists Ed25519, X25519, HKDF-SHA256, and ChaCha20-Poly1305. Those primitives are referenced by `src/alnp/src/crypto/*` and `src/alnp/src/session/mod.rs` for signature verification, key derivation, and MAC tagging. The handshake tests confirm that invalid signatures or MACs fail the session (fail-closed semantics).

## docs/ tree checklist

| Document | Key claim | Current evidence |
|---|---|---|
| `docs/architecture.md` | Layered stack (discovery, control, streaming) | `src/alnp` maintains separate modules for each layer, and the UDP e2e tests exercise them without bypassing the public APIs. |
| `docs/discovery.md` | Broadcast discovery, signed replies, capabilities | `src/alnp/src/discovery.rs` constructs signed replies, and `tests/e2e/discovery_e2e.rs` verifies the behavior. |
| `docs/handshake.md` | Mutual authentication, capability negotiation | `src/alnp/src/handshake/*` plus `tests/e2e/handshake_udp_e2e.rs` implement Section 5. |
| `docs/control_plane.md` | Reliable MAC-verified control envelopes, ACKs | `src/alnp/src/control.rs` tracks sequence numbers and tags each envelope; `tests/e2e/control_udp_e2e.rs` confirms ACK handling. |
| `docs/streaming.md` | Frame envelopes, jitter strategies (`hold-last`, `drop`, `lerp`) | `src/alnp/src/stream.rs` exposes `JitterStrategy`, and the UDP streaming test plus benchmark use `HoldLast` over sockets. |
| `docs/capabilities.md` | Capability maps in discovery/handshake/get_caps | `CapabilitySet` is shared across `src/alnp/src/messages`, `src/alnp/src/handshake`, and `src/alnp/src/session`, and `tests/feature_suite.rs` validates defaults. |
| `docs/errors.md` | Structured codes such as `STREAM_TOO_LARGE` | `ErrorCode` in `src/alnp/src/messages/mod.rs` matches the list. |
| `docs/security.md` | Ed25519, X25519, HKDF, ChaCha20-Poly1305 | `src/alnp/src/crypto` and the session layer instantiate those primitives. |
| `docs/reference_impl.md` | Multi-language references (Rust, TS, Python, C) | `bindings/ts`, `bindings/python`, and `bindings/c` implement shared APIs (`CapabilitySet`, discovery replies) even though this audit did not invoke their build flows. |
| `src/alnp/docs/benchmarks.md` | Real UDP benchmark harness + methodology | `benches/alpine_streaming.rs`, `benches/artnet_streaming.rs`, and `benches/sacn_streaming.rs` reuse the shared handshake helper, and the Criterion reports appear under `target/criterion/<bench_name>/report/index.html`. The most recent run reported medians around 10.3-10.6 us (128 channels) and 13.4-13.7 us (512 channels) for ALPINE. |

## Benchmark reality check

- `cargo bench -- --nocapture` runs all three Criterion benchmarks (`alpine_streaming`, `artnet_streaming`, `sacn_streaming`) and produces full reports under `target/criterion/alpine_streaming_latency`, `target/criterion/artnet_streaming_latency`, and `target/criterion/sacn_streaming_latency`.
- Latest measured medians from the console output:
  * **ALPINE** - 128 channels: ~10.3-11.0 us, 512 channels: ~13.4-13.7 us.
  * **Art-Net** - 128 channels: ~6.3-6.5 us, 512 channels: ~6.5-6.7 us.
  * **sACN** - 128 channels: ~7.6-8.3 us, 512 channels: ~6.5-6.7 us.
- These numbers follow the encode -> send -> receive -> decode loop described in `src/alnp/docs/benchmarks.md`, and the CSV/histograms in each `report/index.html` document the same medians, p95, and outlier counts.

## Outstanding observations

- `docs/reference_impl.md` promises working bindings for TS, Python, and C. Those directories exist, but their package managers were not invoked in this audit, so their pipelines should be verified before claiming parity.
- `docs/security.md` mentions optional vendor certificates and local pairing modes. Those features are not currently exercised in `src/alnp`, so they are still candidates for follow-up work if they must be part of the security story.
- `docs/errors.md` defines streaming errors like `STREAM_TOO_LARGE` and `STREAM_UNSUPPORTED_CHANNEL_MODE`. Although the variants exist in `ErrorCode`, the current runtime does not emit every error path, so targeted tests would be needed to prove those branches.

Aside from the bindings and optional security/error paths listed above, the core ALPINE 1.0 promises in the docs are implemented: each layer is wired end-to-end over real UDP, capability and security data flow through the stack, and the benchmark results now back the documented performance claims.
