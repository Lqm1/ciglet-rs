# Plotting compatibility

All five public Gnuplot helpers are represented by `plotting::Commands` and
native `Figure`: `plotopen` maps to `Figure::open`, `plotopenv` to `open_with`,
`plot` to `Commands::line`, `imagesc` to `Commands::image`, and `plotclose` to
`Figure::close` or owned destruction. Default process selection retains
gnuplot-qt on Unix and uses the standard gnuplot executable on Windows, where
the original helpers were unavailable. Custom executables launch directly with
`-p`, without a shell. `Figure::spawn` accepts configured arguments/environment
and output routing; stdin is always piped and persistence flags are caller-owned.

Commands accept any Rust Write implementation and retain f32/f64 arithmetic,
six-decimal coordinate formatting, scientific matrix formatting, exponent sign
and minimum two-digit width, selected-scalar default x indices, row orientation,
color choices and blue fallback. Locale-independent output corresponds to the
original C locale. Nonfinite points and signed zero remain representable.
Extra x coordinates are ignored; short buffers and oversized dimensions fail
before output. Empty line/matrix command serialization matches C even though
Gnuplot may reject an empty dataset. Stream I/O errors can leave partial output.
Commands flush complete datasets, making them visible without waiting for close.
The public writer_mut method retains arbitrary source figure-stream commands.
Scientific formatting reuses one bounded-size scratch string per matrix.

A waiting worker is established before spawning the child, preventing an
unmanaged child when thread creation fails. After taking its stdin, the child is
transferred through a channel whose send error retains ownership for cleanup.
The worker drains piped stdout/stderr with wait_with_output, avoiding full-pipe
deadlocks. close flushes/closes stdin and waits for main-process status;
close_with_output additionally returns captured bytes. File/inherited output
is not captured. Dropping closes the writer and permits background reaping,
without killing persistent display windows. Explicit close is needed to observe
flush/exit errors. Caller-selected long-running commands can delay close;
captured output remains in memory, so redirect large output to a file.

Unmodified POSIX C command captures cover eight line calls (all six colors,
unknown fallback, optional coordinates, empty data) and three matrix calls
(ordinary, zero rows and zero columns), in both precisions. Inputs include
rounding boundaries, extreme/subnormal magnitudes, signed zero, NaNs and
infinities. GCC 64bit and 32bit regenerate identical bytes. The separate
plotting_lifecycle.c probe verifies original -p launch, command bytes and EOF,
then confirms plotclose left an exited child for waitpid to reap. Rust Linux
tests independently verify that dropped figures disappear from /proc after EOF.

Eight integration tests cover command equality, shape/writer configuration,
write/flush failures, real process stdin/EOF/status/spawn failures, broken pipes,
one MiB each of simultaneous stdout/stderr, drop/reaping, and actual Gnuplot SVG
line/matrix rendering. The real-rendering test is mandatory when
CIGLET_REQUIRE_GNUPLOT is set and uses CIGLET_TEST_GNUPLOT plus the default
executable on PATH. PNG-enabled SVG embeds a raster; a build without PNG support
emits six colored polygons in the plotted-data group. Both represent the tested
2x3 matrix, and the assertion excludes background-axis polygons.

All final 151 integration tests pass on Windows MSVC x86_64/i686, Windows GNU
x86_64 and Linux GNU x86_64/i686 with rendering required. Formatting, Clippy
with warnings denied, and warning-as-error Rustdoc pass. Four original lifecycle
probes (both precisions and bitnesses) confirm the unreaped child and exact -p,
command and EOF behavior. The initial Linux render assertion required an
embedded raster; inspecting the real SVG established its six-polygon fallback,
which is now checked inside the actual plot group. This does not weaken the
requirement that image data be rendered. Final source changes also add output
draining and an independent full-pipe regression, then rerun all five suites.

Validation uses official Gnuplot 6.0.5 Windows binaries and a locally built
Linux file-terminal executable. Published SHA-256 files are checked before use.
These tools remain external test dependencies and add no Cargo dependency.
Headless rendering and -p invocation do not establish interactive Qt/window
behavior on every platform; the stdio lifetime protocol and persistence option
are preserved, and GUI/backend-specific validation remains a separate limitation.

Primary references:

The exact C text fixtures intentionally retain a final sample separator on each
image row. A path-specific Git whitespace attribute exempts that trailing space;
other paths retain their usual checks and fixture byte comparisons remain exact.

- https://git-scm.com/docs/gitattributes#_checking_whitespace_errors

- https://doc.rust-lang.org/stable/std/process/struct.Child.html
- https://doc.rust-lang.org/stable/std/process/struct.ChildStdin.html
- https://doc.rust-lang.org/stable/std/thread/struct.Builder.html
- https://doc.rust-lang.org/stable/std/sync/mpsc/struct.SendError.html
- https://www.gnuplot.info/docs_6.0/loc4148.html
- https://www.gnuplot.info/faq/
- https://www.gnuplot.info/ReleaseNotes_6_0_5.html
