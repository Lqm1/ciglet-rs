#![cfg(all(feature = "c-api", not(target_family = "wasm")))]
use ciglet_rs::c_api::*;
use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    ptr,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temporary(PathBuf);
impl Temporary {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "c-api-figure-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn native(path: &Path) -> Vec<u8> {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        path.as_os_str()
            .encode_wide()
            .flat_map(u16::to_le_bytes)
            .collect()
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        path.as_os_str().as_bytes().to_vec()
    }
}
fn descriptor(bytes: &[u8], encoding: u32) -> CigletRsPath {
    CigletRsPath {
        bytes: bytes.as_ptr(),
        byte_count: bytes.len(),
        encoding,
    }
}
fn options(program: CigletRsPath) -> CigletRsPlotProcessOptions {
    CigletRsPlotProcessOptions {
        program,
        arguments: ptr::null(),
        argument_count: 0,
        environment: ptr::null(),
        environment_count: 0,
        clear_environment: 0,
        directory: ptr::null(),
        stdout_mode: 2,
        stdout_file: ptr::null(),
        stderr_mode: 2,
        stderr_file: ptr::null(),
    }
}
fn state() -> CigletRsPlotExitStatus {
    CigletRsPlotExitStatus {
        success: 99,
        has_code: 99,
        code: 99,
        raw_status: 99,
        raw_format: 99,
    }
}
fn spawn(directory: &Temporary, mode: &str) -> *mut CigletRsFigure {
    let program = native(&std::env::current_exe().unwrap());
    let directory_bytes = native(&directory.0);
    let argument_bytes: [&[u8]; 3] = [b"--exact", b"subprocess_fixture", b"--nocapture"];
    let arguments: Vec<_> = argument_bytes
        .into_iter()
        .map(|v| descriptor(v, 0))
        .collect();
    let directory_value = descriptor(&directory_bytes, 1);
    let mode_value = descriptor(mode.as_bytes(), 0);
    let environment = [
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_FIGURE_DIRECTORY", 0),
            value: &directory_value,
        },
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_FIGURE_MODE", 0),
            value: &mode_value,
        },
    ];
    let mut config = options(descriptor(&program, 1));
    config.arguments = arguments.as_ptr();
    config.argument_count = arguments.len();
    config.environment = environment.as_ptr();
    config.environment_count = environment.len();
    let mut figure = ptr::null_mut();
    // SAFETY: Nested tables/bytes and exclusive ownership slot remain stable through synchronous spawn.
    unsafe {
        assert_eq!(ciglet_rs_figure_spawn(config, &mut figure), 0);
    }
    figure
}
fn until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(Instant::now() < deadline, "child did not finish");
        thread::sleep(Duration::from_millis(10));
    }
}
unsafe fn consume(mut owner: *mut CigletRsBytes) -> Vec<u8> {
    let mut length = 0;
    // SAFETY: Live byte owner and independently writable buffers, released once.
    unsafe {
        assert_eq!(ciglet_rs_bytes_length(owner, &mut length), 0);
        let mut bytes = vec![0; length];
        assert_eq!(
            ciglet_rs_bytes_copy(owner, 0, bytes.as_mut_ptr(), length),
            0
        );
        assert_eq!(ciglet_rs_bytes_release(&mut owner), 0);
        bytes
    }
}
#[test]
fn subprocess_fixture() {
    let Some(directory) = std::env::var_os("CIGLET_ABI_FIGURE_DIRECTORY") else {
        return;
    };
    let directory = PathBuf::from(directory);
    let mode = std::env::var("CIGLET_ABI_FIGURE_MODE").unwrap();
    if mode == "options" {
        assert_eq!(std::env::current_dir().unwrap(), directory);
        assert!(std::env::var_os("PATH").is_none());
        assert!(std::env::var_os("CIGLET_ABI_REMOVE").is_none());
        io::stdout().write_all(b"Options output\n").unwrap();
        io::stderr().write_all(b"Options error\n").unwrap();
    }
    if mode == "broken" {
        fs::write(directory.join("ready"), b"ready").unwrap();
        std::process::exit(7);
    }
    if mode == "large" {
        io::stdout().write_all(&vec![b'A'; 1024 * 1024]).unwrap();
        io::stderr().write_all(&vec![b'B'; 1024 * 1024]).unwrap();
    }
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).unwrap();
    fs::write(directory.join("commands"), bytes).unwrap();
    if mode == "status" {
        std::process::exit(7);
    }
    if mode == "wide" {
        std::process::exit(i32::MIN + 7);
    }
    #[cfg(unix)]
    if mode == "signal" {
        unsafe extern "C" {
            fn raise(signal: i32) -> i32;
        }
        // SAFETY: Test child terminates itself with SIGTERM, never its parent.
        unsafe {
            raise(15);
        }
        panic!("SIGTERM should terminate the child");
    }
    if mode == "drop" {
        thread::sleep(Duration::from_millis(100));
    }
    fs::write(directory.join("finished"), b"finished").unwrap();
}

#[test]
fn process_stream_uses_both_original_transcripts() {
    let directory = Temporary::new();
    let mut figure = spawn(&directory, "capture");
    let mut expected = Vec::new();
    // SAFETY: Retained figure and sample/array owners remain independent and stable until close.
    unsafe {
        let mut id = 0;
        assert_eq!(ciglet_rs_figure_id(figure, &mut id), 0);
        assert!(id > 0);
        macro_rules! transcript {
            ($t:ty,$line:ident,$image:ident,$array:ident,$release:ident,$fixture:expr) => {{
                let y: Vec<$t> = [
                    0.0,
                    -0.0,
                    1.23456789,
                    -0.0000005,
                    1e20,
                    1e-20,
                    <$t>::MAX as f64,
                    <$t>::from_bits(1) as f64,
                    f64::NAN,
                    -f64::NAN,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                ]
                .into_iter()
                .map(|v| v as $t)
                .collect();
                let x: Vec<$t> = (0..12).map(|i| (i as f64 * 0.25 - 1.0) as $t).collect();
                for (i, color) in b"brgkcy?".iter().copied().enumerate() {
                    let (xp, xc) = if i % 2 == 0 {
                        (ptr::null(), 0)
                    } else {
                        (x.as_ptr(), 13)
                    };
                    assert_eq!($line(figure, xp, xc, y.as_ptr(), 12, color), 0);
                }
                assert_eq!($line(figure, ptr::null(), 0, ptr::null(), 0, b'b'), 0);
                for (rows, columns) in [(2, 6), (0, 3), (2, 0)] {
                    let mut array = ptr::null_mut();
                    assert_eq!(
                        $array(rows, columns, y.as_ptr(), rows * columns, &mut array),
                        0
                    );
                    assert_eq!($image(figure, array), 0);
                    assert_eq!($release(&mut array), 0);
                }
                expected.extend_from_slice($fixture);
            }};
        }
        transcript!(
            f32,
            ciglet_rs_figure_line_f32,
            ciglet_rs_figure_image_f32,
            ciglet_rs_array_from_slice_f32,
            ciglet_rs_array_release_f32,
            include_bytes!("fixtures/plotting-float.txt")
        );
        transcript!(
            f64,
            ciglet_rs_figure_line_f64,
            ciglet_rs_figure_image_f64,
            ciglet_rs_array_from_slice_f64,
            ciglet_rs_array_release_f64,
            include_bytes!("fixtures/plotting-double.txt")
        );
        let raw = b"set title 'custom'\n\0\xff";
        assert_eq!(
            ciglet_rs_figure_configuration(figure, raw.as_ptr(), raw.len()),
            0
        );
        expected.extend_from_slice(raw);
        assert_eq!(ciglet_rs_figure_configuration(figure, ptr::null(), 0), 0);
        let mut exit = state();
        assert_eq!(ciglet_rs_figure_close(&mut figure, &mut exit), 0);
        assert!(figure.is_null());
        assert_eq!((exit.success, exit.has_code, exit.code), (1, 1, 0));
    }
    assert_eq!(fs::read(directory.file("commands")).unwrap(), expected);
}

#[test]
fn large_capture_is_drained_while_writing_and_nonzero_exit_is_not_an_error() {
    let directory = Temporary::new();
    let mut figure = spawn(&directory, "large");
    let input = vec![b'C'; 1024 * 1024];
    let mut out = ptr::null_mut();
    let mut err = ptr::null_mut();
    let mut exit = state();
    // SAFETY: Independent initialized input, retained owner and three exclusive result slots.
    unsafe {
        assert_eq!(
            ciglet_rs_figure_configuration(figure, input.as_ptr(), input.len()),
            0
        );
        assert_eq!(
            ciglet_rs_figure_close_output(&mut figure, &mut exit, &mut out, &mut err),
            0
        );
        assert!(figure.is_null());
        assert_eq!(exit.success, 1);
        assert_eq!(
            consume(out).iter().filter(|&&v| v == b'A').count(),
            1024 * 1024
        );
        assert_eq!(
            consume(err).iter().filter(|&&v| v == b'B').count(),
            1024 * 1024
        );
        assert_eq!(fs::read(directory.file("commands")).unwrap(), input);
        for mode in ["status", "wide"] {
            let mut figure = spawn(&directory, mode);
            assert_eq!(ciglet_rs_figure_close(&mut figure, &mut exit), 0);
            assert!(figure.is_null());
            assert_eq!(exit.success, 0);
            assert_eq!(exit.has_code, 1);
            #[cfg(windows)]
            {
                assert_eq!(exit.raw_format, 1);
                let expected = if mode == "status" { 7 } else { i32::MIN + 7 };
                assert_eq!(exit.code, expected);
                assert_eq!(exit.raw_status, expected as u32);
            }
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                assert_eq!(exit.raw_format, 0);
                assert_eq!(exit.code, 7);
                assert_eq!(
                    std::process::ExitStatus::from_raw(exit.raw_status as i32).code(),
                    Some(7)
                );
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn signal_exit_retains_raw_wait_status() {
    let directory = Temporary::new();
    let mut figure = spawn(&directory, "signal");
    let mut exit = state();
    // SAFETY: Live exclusive owner slot and independent initialized status output.
    unsafe {
        assert_eq!(ciglet_rs_figure_close(&mut figure, &mut exit), 0);
    }
    use std::os::unix::process::ExitStatusExt;
    assert_eq!(
        (exit.success, exit.has_code, exit.code, exit.raw_format),
        (0, 0, 0, 0)
    );
    assert_eq!(
        std::process::ExitStatus::from_raw(exit.raw_status as i32).signal(),
        Some(15)
    );
}

#[test]
fn release_closes_stdin_without_killing_or_waiting_and_reaps() {
    let directory = Temporary::new();
    let mut figure = spawn(&directory, "drop");
    let mut id = 0;
    // SAFETY: Owner has no concurrent users and all output slots are independent.
    unsafe {
        assert_eq!(ciglet_rs_figure_id(figure, &mut id), 0);
        assert_eq!(
            ciglet_rs_figure_configuration(figure, b"custom\n".as_ptr(), 7),
            0
        );
        assert_eq!(ciglet_rs_figure_release(&mut figure), 0);
        assert!(figure.is_null());
        assert_eq!(ciglet_rs_figure_release(&mut figure), 0);
    }
    until(|| directory.file("finished").exists());
    assert_eq!(fs::read(directory.file("commands")).unwrap(), b"custom\n");
    #[cfg(target_os = "linux")]
    until(|| !PathBuf::from(format!("/proc/{id}")).exists());
}

#[test]
fn invalid_outputs_preserve_owner_and_broken_pipe_consumes_on_close() {
    let directory = Temporary::new();
    let mut figure = spawn(&directory, "capture");
    let original = figure;
    let mut exit = state();
    let mut out = ptr::null_mut();
    // SAFETY: Null pointers are validated without dereference; other inputs are independent live storage.
    unsafe {
        assert_eq!(ciglet_rs_figure_close(&mut figure, ptr::null_mut()), 1);
        assert_eq!(figure, original);
        assert_eq!(
            ciglet_rs_figure_close_output(&mut figure, &mut exit, &mut out, ptr::null_mut()),
            1
        );
        assert_eq!(figure, original);
        assert_eq!(ciglet_rs_figure_id(figure, ptr::null_mut()), 1);
        assert_eq!(
            ciglet_rs_figure_line_f64(figure, [1.0].as_ptr(), 1, [1.0, 2.0].as_ptr(), 2, b'b'),
            2
        );
        assert_eq!(ciglet_rs_figure_image_f64(figure, ptr::null()), 1);
        assert_eq!(ciglet_rs_figure_configuration(figure, ptr::null(), 1), 1);
        assert_eq!(ciglet_rs_figure_close(&mut figure, &mut exit), 0);
    }
    assert!(fs::read(directory.file("commands")).unwrap().is_empty());
    figure = spawn(&directory, "broken");
    until(|| directory.file("ready").exists());
    let input = vec![b'x'; 1024 * 1024];
    // SAFETY: Stable input and exclusive owner/results; child exited intentionally.
    unsafe {
        assert_eq!(
            ciglet_rs_figure_configuration(figure, input.as_ptr(), input.len()),
            3
        );
        let mut err = ptr::null_mut();
        let result = ciglet_rs_figure_close_output(&mut figure, &mut exit, &mut out, &mut err);
        assert!(figure.is_null());
        if result == 0 {
            assert_eq!(exit.code, 7);
            let _ = consume(out);
            let _ = consume(err);
        } else {
            assert_eq!(result, 3);
            assert!(out.is_null() && err.is_null());
        }
    }
}

#[test]
fn invalid_spawn_never_opens_files_and_missing_program_retains_output() {
    let directory = Temporary::new();
    let file = directory.file("retained");
    fs::write(&file, b"retained").unwrap();
    let file_bytes = native(&file);
    let file_path = descriptor(&file_bytes, 1);
    let mut config = options(descriptor(b"missing-program-figure-fixture", 0));
    config.stdout_mode = 3;
    config.stdout_file = &file_path;
    config.stderr_mode = 99;
    let mut owner = ptr::null_mut();
    // SAFETY: Stable nested file descriptor and exclusive owner slot; invalid mode prevents mutation.
    unsafe {
        assert_eq!(ciglet_rs_figure_spawn(config, &mut owner), 3);
        assert!(owner.is_null());
        assert_eq!(fs::read(&file).unwrap(), b"retained");
        config.stderr_mode = 2;
        assert_eq!(ciglet_rs_figure_spawn(config, ptr::null_mut()), 1);
        assert_eq!(fs::read(&file).unwrap(), b"retained");
        assert_eq!(ciglet_rs_figure_open_with(config.program, &mut owner), 3);
        assert!(owner.is_null());
        config.clear_environment = 2;
        assert_eq!(ciglet_rs_figure_spawn(config, &mut owner), 3);
        config.clear_environment = 0;
        config.argument_count = 1;
        assert_eq!(ciglet_rs_figure_spawn(config, &mut owner), 1);
    }
}

#[test]
fn custom_directory_environment_and_file_output_modes() {
    let directory = Temporary::new();
    let program = native(&std::env::current_exe().unwrap());
    let directory_bytes = native(&directory.0);
    let directory_value = descriptor(&directory_bytes, 1);
    let mode = descriptor(b"options", 0);
    let arguments = [
        descriptor(b"--exact", 0),
        descriptor(b"subprocess_fixture", 0),
        descriptor(b"--nocapture", 0),
    ];
    let environment = [
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_FIGURE_DIRECTORY", 0),
            value: &directory_value,
        },
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_FIGURE_MODE", 0),
            value: &mode,
        },
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_REMOVE", 0),
            value: &mode,
        },
        CigletRsPlotEnvironment {
            key: descriptor(b"CIGLET_ABI_REMOVE", 0),
            value: ptr::null(),
        },
    ];
    let stdout_file = directory.file("stdout");
    let stderr_file = directory.file("stderr");
    let stdout_bytes = native(&stdout_file);
    let stderr_bytes = native(&stderr_file);
    let stdout_value = descriptor(&stdout_bytes, 1);
    let stderr_value = descriptor(&stderr_bytes, 1);
    fs::write(&stdout_file, b"must truncate").unwrap();
    fs::write(&stderr_file, b"must retain\n").unwrap();
    let mut config = options(descriptor(&program, 1));
    config.arguments = arguments.as_ptr();
    config.argument_count = arguments.len();
    config.environment = environment.as_ptr();
    config.environment_count = environment.len();
    config.clear_environment = 1;
    config.directory = &directory_value;
    config.stdout_mode = 3;
    config.stdout_file = &stdout_value;
    config.stderr_mode = 4;
    config.stderr_file = &stderr_value;
    let mut figure = ptr::null_mut();
    let mut exit = state();
    let mut out = ptr::null_mut();
    let mut err = ptr::null_mut();
    // SAFETY: All nested counted descriptors remain stable, and every output slot is independent.
    unsafe {
        assert_eq!(ciglet_rs_figure_spawn(config, &mut figure), 0);
        assert_eq!(
            ciglet_rs_figure_close_output(&mut figure, &mut exit, &mut out, &mut err),
            0
        );
        assert_eq!(exit.success, 1);
        assert!(consume(out).is_empty());
        assert!(consume(err).is_empty());
        let output = fs::read_to_string(&stdout_file).unwrap();
        let error = fs::read_to_string(&stderr_file).unwrap();
        assert!(output.contains("Options output\n") && !output.contains("must truncate"));
        assert_eq!(error, "must retain\nOptions error\n");
        config.stdout_mode = 1;
        config.stdout_file = ptr::null();
        config.stderr_mode = 1;
        config.stderr_file = ptr::null();
        assert_eq!(ciglet_rs_figure_spawn(config, &mut figure), 0);
        assert_eq!(
            ciglet_rs_figure_close_output(&mut figure, &mut exit, &mut out, &mut err),
            0
        );
        assert_eq!(exit.success, 1);
        assert!(consume(out).is_empty());
        assert!(consume(err).is_empty());
        assert_eq!(fs::read_to_string(stdout_file).unwrap(), output);
        assert_eq!(fs::read_to_string(stderr_file).unwrap(), error);
    }
}

#[test]
fn real_gnuplot_default_and_custom_launch_render_svg() {
    let Some(program) = std::env::var_os("CIGLET_TEST_GNUPLOT") else {
        assert!(
            std::env::var_os("CIGLET_REQUIRE_GNUPLOT").is_none(),
            "Gnuplot executable required"
        );
        return;
    };
    let directory = Temporary::new();
    let program_bytes = native(Path::new(&program));
    let program = descriptor(&program_bytes, 1);
    // SAFETY: Stable counted program/path bytes, independent owner/status slots and typed arrays.
    unsafe {
        for mode in 0..3 {
            let mut figure = ptr::null_mut();
            match mode {
                0 => assert_eq!(ciglet_rs_figure_open(&mut figure), 0),
                1 => assert_eq!(ciglet_rs_figure_open_with(program, &mut figure), 0),
                _ => assert_eq!(ciglet_rs_figure_spawn(options(program), &mut figure), 0),
            }
            let line_file = directory.file(&format!("line-{mode}.svg"));
            let image_file = directory.file(&format!("image-{mode}.svg"));
            let escaped = |p: &Path| p.to_str().unwrap().replace('\\', "/").replace('"', "\\\"");
            let commands = format!(
                "set terminal svg size 320,240\nset output \"{}\"\n",
                escaped(&line_file)
            );
            assert_eq!(
                ciglet_rs_figure_configuration(figure, commands.as_ptr(), commands.len()),
                0
            );
            assert_eq!(
                ciglet_rs_figure_line_f64(
                    figure,
                    ptr::null(),
                    0,
                    [0.0, 1.0, 0.0, -1.0].as_ptr(),
                    4,
                    b'r'
                ),
                0
            );
            let commands = format!("set output \"{}\"\n", escaped(&image_file));
            assert_eq!(
                ciglet_rs_figure_configuration(figure, commands.as_ptr(), commands.len()),
                0
            );
            let mut array = ptr::null_mut();
            assert_eq!(
                ciglet_rs_array_from_slice_f32(
                    2,
                    3,
                    [0.0, 1.0, 2.0, 3.0, 4.0, 5.0].as_ptr(),
                    6,
                    &mut array
                ),
                0
            );
            assert_eq!(ciglet_rs_figure_image_f32(figure, array), 0);
            assert_eq!(ciglet_rs_array_release_f32(&mut array), 0);
            assert_eq!(
                ciglet_rs_figure_configuration(figure, b"unset output\n".as_ptr(), 13),
                0
            );
            let mut exit = state();
            assert_eq!(ciglet_rs_figure_close(&mut figure, &mut exit), 0);
            assert_eq!(exit.success, 1);
            assert!(figure.is_null());
            let line = fs::read_to_string(line_file).unwrap();
            let image = fs::read_to_string(image_file).unwrap();
            assert!(line.contains("<svg") && line.contains("rgb(255,   0,   0)"));
            let plotted = image
                .split("id=\"gnuplot_plot_1\"")
                .nth(1)
                .unwrap()
                .split("</g>")
                .next()
                .unwrap();
            assert!(plotted.contains("<image ") || plotted.matches("<polygon ").count() == 6);
        }
    }
}
