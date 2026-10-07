# Remaining Gnuplot interface audit

The pinned ciglet header lines 1310-1392 contain five public functions under
`_POSIX_C_SOURCE >= 2`: plotopenv, plotopen, plot, imagesc and plotclose.
The public figure configuration owns one FILE stream. These functions remain
unimplemented; they are part of full dependency scope even though SHIRO does
not call them and the original excludes Windows.

- plotopenv launches the selected executable directly with argument `-p`,
  connecting a pipe to its stdin. plotopen selects `gnuplot-qt`.
- plot writes a line plot command with blue/red/green/black/cyan/yellow colors.
  Unknown color codes select blue. Optional x coordinates default to selected-
  scalar casts of indices. Each coordinate uses C `%f` output, then `e` ends data.
- imagesc writes x/y ranges from zero through columns/rows minus one, sets map
  view, and sends row-major matrix values with C `%5.3e` formatting and `e`.
- plotclose closes the stream and releases figure storage. No explicit flush is
  performed before closing, so buffering/close semantics affect visibility.

The source does not check allocation, pipe/fork/dup2/fdopen failures and does not
retain/reap the child PID. A Rust implementation must own stream/process state,
propagate I/O/spawn errors, and determine child completion behavior without
discarding persistent-window functionality. It must preserve finite f32/f64
coordinates, matrix orientation, custom executable selection, default colors
and caller access to write additional Gnuplot commands through the owned stream.
Process launch belongs to native hosts; command serialization should remain
separable for capture testing and future byte-oriented WASM integration.

Before acceptance, compare captured commands to the unchanged POSIX C helper,
test real subprocess stdin/EOF/status behavior and failing writes, and verify
actual Gnuplot rendering through an available file terminal. A fake process alone
does not prove rendering. Process APIs and formatter behavior require primary
documentation checks before implementation.

Primary documentation retrieved through Context7/Mintlify confirms that Rust
Child has no Drop-based kill/reap behavior, while dropping ChildStdin closes its
pipe. Child::wait closes only stdin still held by that Child; an extracted writer
must be dropped separately. Waiting is required to release exited child resources
on Unix. Gnuplot's official persist/FAQ documentation states that `-p` permits
display windows to outlive the main program, and does not affect file terminals.
The older Qt terminal page describes persistence differently, so test real
installed backend behavior instead of inferring lifecycle solely from that page.
No Gnuplot executable was found on the current WSL PATH during this audit.

References:

- https://doc.rust-lang.org/stable/std/process/struct.Child.html
- https://doc.rust-lang.org/stable/std/process/struct.ChildStdin.html
- https://doc.rust-lang.org/stable/std/process/index.html
- https://www.gnuplot.info/docs_6.0/loc4148.html
- https://www.gnuplot.info/faq/
- http://www.gnuplot.info/docs/loc22181.html
