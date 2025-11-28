# Changelog

All notable changes to ALPINE will be documented in this file.

## [1.2.0] - 2025-12-04
- Introduce language-specific SDK layers (`src/alnp/src/sdk`, `bindings/ts/src/sdk`, `bindings/python/src/alnp/sdk`, `bindings/cpp/sdk`) with ergonomic APIs (`connect`, `send_frame`, `control`, keepalive) that now select stream profiles.
- Add Stream Profiles (Auto/Realtime/Install) that compile into deterministic `config_id`s, validate weights, and cannot be changed once streaming starts; `start_stream` binds the profile.
- Added tests covering profile validation failures, config_id determinism, and immutability once streaming commences.
- Update docs/README to position those SDKs as the recommended application entry points while keeping the auto-generated bindings stable for embedded use.
- Keep the existing release artifacts, embedded CI job, and documentation policy so every platform continues to ship consistent behavior guarantees.

## [1.0.10] - 2025-11-27
- Publish the Rust crate as `alpine-protocol-rs` so we can continue releasing from the `alpine-core` repo even though the old crate name was owned elsewhere.
- Update release scripts to copy the new `alpine-protocol-rs-*.crate` so the artifact matches the crates.io name while clients still include `alpine` in their namespaces.
- Keep shipping docs, the GHCR C package, and the embedded-friendly CI job so every release bundles docs + binaries with the constrained build checks.

## [1.0.9] - 2025-11-27
- Add the `embedded` build profile (`#define ALPINE_EMBEDDED`) so the C++ helper compiles with no exceptions, RTTI, or heap allocations.
- Validate that mode via `.github/workflows/embedded.yml` and `scripts/build_embedded_cpp.sh`, which runs with the ESP32-safe flag set for every push/PR.
- Document the embedded path in the README/docs so constrained targets get the same great API as desktop builds.

## [1.0.8] - 2025-11-27
- Provide the C++ helper header (`bindings/cpp/alnp.hpp`) so C++ projects can include ALPINE without touching the raw C structs.
- Deliver the new header along with `README.md`, `SPEC.md`, and `docs/` in each release asset so documentation and libs travel together.
- Keep the GHCR C package, TypeScript, Python, and Rust releases aligned under the `v1.0.8` tag.

## [1.0.7] - 2025-11-27
- Keep the crate name `alpine-core` for crates.io while exposing the library as `alpine` so existing tests and consumers can still import `alpine::...`.
- Restore `libalpine.a` as the C artifact while keeping the GHCR image packaging and docs bundle unchanged.
- Continue shipping docs + artifacts together so release pages always include README/SPEC/docs and the tarball on GHCR.

## [1.0.6] - 2025-11-27
- Rename the Rust crate to `alpine-core` and ship it as `alpine-core-1.0.6` so future updates belong to the alpine-core organization.
- Update the static library export to `libalpine-core-*.a` so the C bindings still match the crate name.
- Keep docs, GHCR C package, and release notes flowing with the new tag.

## [1.0.5] - 2025-11-27
- Publish the C tarball as both a release asset and a GHCR package so it is easy to download and use.
- Bundle `README.md`, `SPEC.md`, and `docs/` into each release asset so the documentation always travels with every package.
- Keep TS, Python, and Rust manifests in sync with the `v1.0.5` tag so CI redeploys everything cleanly.

## [1.0.4] - 2025-11-27
- Target `crates-io` explicitly when publishing Rust so Cargo knows which registry to use.
- Refresh every binding manifest/tag to `1.0.4` so the next release has new artifacts.
- Confirm C/TS/Python jobs still upload via GitHub Packages with the new permissions.

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

[1.0.11]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.11
[1.0.10]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.10
[1.0.9]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.9
[1.0.8]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.8
[1.0.7]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.7
[1.0.6]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.6
[1.0.5]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.5
[1.0.4]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.4
[1.0.3]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.3
[1.0.2]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.2
[1.0.0]: https://github.com/alpine-core/Authenticated-Lighting-Protocol/releases/tag/v1.0.0
