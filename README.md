# ciglet-rs

A Rust reimplementation of the complete public ciglet signal-processing functionality. Upstream: https://github.com/Sleepwalking/ciglet.

The port is in progress. Statistics, precise window functions, and centered zero-padded frame extraction are implemented. The remaining DSP algorithms, approximation variants, audio I/O, synthesis, and plotting integrations are pending.

```powershell
cargo test
cargo test --target i686-pc-windows-msvc
cargo clippy --all-targets -- -D warnings
```

This Rust project uses GPL-3.0-or-later as requested for the three-project port. The original ciglet BSD 3-Clause attribution and license are retained in `LICENSES/ciglet-BSD-3-Clause.txt`. No original C implementation is linked into the Rust library.
