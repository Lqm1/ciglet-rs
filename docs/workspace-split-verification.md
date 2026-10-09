# Workspace split verification

This checkpoint verifies the native/C/WASM package split. Earlier acceptance
reports describe the previous single-package layout and remain historical evidence.

## Scope

The repository has three flat workspace members: `ciglet-rs`,
`ciglet-rs-capi` and `ciglet-rs-wasm`. All were initialized with `cargo new --lib`.
The native API contains the computations; adapters call that API directly.
The native package forbids unsafe code and has no binding-specific dependencies.
Adapters use a path dependency within this workspace. No sibling repository is
needed to build this project.

The migration preserves native computations, fixtures and license notices.
Three previously internal checked helpers are now public for the adapters:
`vector::validate`, `vector::buffer` and `resampling::output_length`.
Regenerated cbindgen 0.29.4 headers match the checked-in header byte for byte.
C symbols and layouts are unchanged. The C artifact base name is
`ciglet_rs_capi`; generated JavaScript keeps `ciglet_rs.js` via `--out-name`.

## Executed checks

The split working tree passed `cargo test --locked --workspace -- --include-ignored`
for x86_64-pc-windows-msvc, i686-pc-windows-msvc,
x86_64-pc-windows-gnu, x86_64-unknown-linux-gnu and i686-unknown-linux-gnu.
Each target passed 463 tests, with zero failures and zero ignored tests.

All seven independent C caller programs passed on each of those five targets.
All seven Python caller programs passed on each of the three 64-bit targets.
The C programs were compiled with optimization and assertions enabled.
Python 32-bit execution was not tested.

Native workspace and wasm32 adapter Clippy checks passed with warnings denied.
Rustdoc passed with warnings denied, and formatting and whitespace checks passed.
The generated Node.js module passed all 40 portable verification families.
Three independently loaded C DLLs and three independently instantiated WASM
modules also passed the SHIRO real-speech feature/model/training composition.

The new web-target JavaScript glue is byte-identical to the previously browser-
verified glue. A fresh browser execution during this migration was unavailable
because the browser automation connection timed out. The new WASM computations
were executed through Node.js; this checkpoint does not claim a fresh browser run.

Scratch execution logs are held in the migration task workspace under
`work/split-native-*`, `work/split-test-*`, `work/split-callers-*` and
`work/split-node-*`. These checks ran on the split working tree before its
publication commit; old HEAD values in runner logs are baseline identifiers,
not a claim that the old commits already contain the split.
