use ciglet_rs::{
    array::Array2,
    plotting::{Color, Commands, Figure},
    statistics::Scalar,
};
use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

fn transcript<T: Scalar>() -> Vec<u8> {
    let maximum = if std::mem::size_of::<T>() == 4 {
        f32::MAX as f64
    } else {
        f64::MAX
    };
    let tiny = if std::mem::size_of::<T>() == 4 {
        f32::from_bits(1) as f64
    } else {
        f64::from_bits(1)
    };
    let y: Vec<T> = [
        0.0,
        -0.0,
        1.23456789,
        -0.0000005,
        1e20,
        1e-20,
        maximum,
        tiny,
        f64::NAN,
        -f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ]
    .into_iter()
    .map(|x| T::from_f64(x).unwrap())
    .collect();
    let x: Vec<T> = (0..12)
        .map(|i| T::from_f64(i as f64 * 0.25 - 1.0).unwrap())
        .collect();
    let mut commands = Commands::new(Vec::new());
    for (index, color) in ['b', 'r', 'g', 'k', 'c', 'y', '?'].into_iter().enumerate() {
        commands
            .line(
                if index % 2 == 0 { None } else { Some(&x) },
                &y,
                Color::from(color),
            )
            .unwrap();
    }
    commands.line(None, &y[..0], Color::Blue).unwrap();
    commands
        .image(&Array2::from_row_major(2, 6, y).unwrap())
        .unwrap();
    commands
        .image(&Array2::<T>::from_row_major(0, 3, vec![]).unwrap())
        .unwrap();
    commands
        .image(&Array2::<T>::from_row_major(2, 0, vec![]).unwrap())
        .unwrap();
    commands.into_inner()
}

#[test]
fn unchanged_posix_c_commands() {
    assert_eq!(
        transcript::<f32>(),
        include_bytes!("../../../tests/fixtures/plotting-float.txt")
    );
    assert_eq!(
        transcript::<f64>(),
        include_bytes!("../../../tests/fixtures/plotting-double.txt")
    );
}

struct FailingWriter {
    flush: bool,
}
impl Write for FailingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.flush {
            Ok(bytes.len())
        } else {
            Err(io::ErrorKind::BrokenPipe.into())
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::ErrorKind::BrokenPipe.into())
    }
}
#[test]
fn command_shapes_stream_access_and_io_errors() {
    let mut commands = Commands::new(Vec::new());
    assert_eq!(
        commands
            .line(Some(&[0.0]), &[1.0, 2.0], Color::Red)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(commands.writer_mut().is_empty());
    commands
        .writer_mut()
        .write_all(b"set title 'custom'\n")
        .unwrap();
    commands
        .line(Some(&[4.0, 9.0]), &[2.0], Color::Red)
        .unwrap();
    assert_eq!(
        String::from_utf8(commands.into_inner()).unwrap(),
        "set title 'custom'\nplot '-' with lines lc rgb \"red\"\n4.000000 2.000000\ne\n"
    );
    for flush in [false, true] {
        let mut commands = Commands::new(FailingWriter { flush });
        assert_eq!(
            commands
                .line(None, &[1.0_f64], Color::Blue)
                .unwrap_err()
                .kind(),
            io::ErrorKind::BrokenPipe
        );
        assert_eq!(
            commands
                .image(&Array2::from_row_major(1, 1, vec![1.0]).unwrap())
                .unwrap_err()
                .kind(),
            io::ErrorKind::BrokenPipe
        );
    }
    let excessive = Array2::<f64>::from_row_major(0, i32::MAX as usize + 1, vec![]).unwrap();
    assert!(Commands::new(Vec::new()).image(&excessive).is_err());
}

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temporary(PathBuf);
impl Temporary {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "ciglet-plot-{}-{}",
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
fn child_command(directory: &Temporary, mode: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", "subprocess_fixture", "--nocapture"])
        .env("CIGLET_PLOT_FIXTURE", &directory.0)
        .env("CIGLET_PLOT_MODE", mode)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}
fn until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(Instant::now() < deadline, "plot child did not finish");
        thread::sleep(Duration::from_millis(10));
    }
}

// Executed by a spawned copy of this test binary; ordinary suite runs do nothing.
#[test]
fn subprocess_fixture() {
    let Some(directory) = std::env::var_os("CIGLET_PLOT_FIXTURE") else {
        return;
    };
    let directory = PathBuf::from(directory);
    let mode = std::env::var("CIGLET_PLOT_MODE").unwrap();
    if mode == "broken" {
        fs::write(directory.join("ready"), b"ready").unwrap();
        std::process::exit(7);
    }
    if mode == "output" {
        io::stdout().write_all(&vec![b'A'; 1024 * 1024]).unwrap();
        io::stderr().write_all(&vec![b'B'; 1024 * 1024]).unwrap();
    }
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).unwrap();
    fs::write(directory.join("commands"), bytes).unwrap();
    if mode == "status" {
        std::process::exit(7);
    }
    if mode == "drop" {
        thread::sleep(Duration::from_millis(100));
    }
    fs::write(directory.join("finished"), b"finished").unwrap();
}

#[test]
fn native_pipe_close_status_and_spawn_error() {
    let directory = Temporary::new();
    let mut figure = Figure::spawn(&mut child_command(&directory, "capture")).unwrap();
    assert!(figure.id() > 0);
    figure
        .commands_mut()
        .line(None, &[1.0_f64, 2.0], Color::Green)
        .unwrap();
    assert!(figure.close().unwrap().success());
    assert_eq!(
        fs::read(directory.file("commands")).unwrap(),
        b"plot '-' with lines lc rgb \"green\"\n0.000000 1.000000\n1.000000 2.000000\ne\n"
    );
    let figure = Figure::spawn(&mut child_command(&directory, "status")).unwrap();
    assert_eq!(figure.close().unwrap().code(), Some(7));
    assert!(Figure::open_with(directory.file("missing-executable")).is_err());
}

#[test]
fn drop_sends_eof_without_killing_child_and_reaps() {
    let directory = Temporary::new();
    let mut figure = Figure::spawn(&mut child_command(&directory, "drop")).unwrap();
    let id = figure.id();
    figure
        .commands_mut()
        .writer_mut()
        .write_all(b"custom buffered command\n")
        .unwrap();
    drop(figure);
    until(|| directory.file("finished").exists());
    assert_eq!(
        fs::read(directory.file("commands")).unwrap(),
        b"custom buffered command\n"
    );
    #[cfg(target_os = "linux")]
    until(|| !PathBuf::from(format!("/proc/{id}")).exists());
    #[cfg(not(target_os = "linux"))]
    let _ = id;
}

#[test]
fn actual_broken_pipe_is_reported() {
    let directory = Temporary::new();
    let mut figure = Figure::spawn(&mut child_command(&directory, "broken")).unwrap();
    until(|| directory.file("ready").exists());
    assert!(
        figure
            .commands_mut()
            .writer_mut()
            .write_all(&vec![b'x'; 1024 * 1024])
            .is_err()
    );
    // An empty remaining buffer can flush successfully even after this write
    // failed; either the flush error or the actual exit status must be visible.
    if let Ok(status) = figure.close() {
        assert_eq!(status.code(), Some(7));
    }
}

#[test]
fn simultaneous_large_stdout_stderr_and_stdin() {
    let directory = Temporary::new();
    let mut command = child_command(&directory, "output");
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut figure = Figure::spawn(&mut command).unwrap();
    figure
        .commands_mut()
        .writer_mut()
        .write_all(b"consume input\n")
        .unwrap();
    let output = figure.close_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        output.stdout.iter().filter(|&&value| value == b'A').count(),
        1024 * 1024
    );
    assert_eq!(
        output.stderr.iter().filter(|&&value| value == b'B').count(),
        1024 * 1024
    );
    assert_eq!(
        fs::read(directory.file("commands")).unwrap(),
        b"consume input\n"
    );
}

#[test]
fn real_gnuplot_svg_rendering() {
    let Some(program) = std::env::var_os("CIGLET_TEST_GNUPLOT") else {
        assert!(
            std::env::var_os("CIGLET_REQUIRE_GNUPLOT").is_none(),
            "Gnuplot test executable required"
        );
        return;
    };
    let directory = Temporary::new();
    let mut figure = Figure::open_with(program).unwrap();
    let path = directory
        .0
        .to_str()
        .unwrap()
        .replace('\\', "/")
        .replace('"', "\\\"");
    writeln!(figure.commands_mut().writer_mut(), "cd \"{path}\"").unwrap();
    figure
        .commands_mut()
        .writer_mut()
        .write_all(b"set terminal svg size 320,240\nset output 'line.svg'\n")
        .unwrap();
    figure
        .commands_mut()
        .line(None, &[0.0, 1.0, 0.0, -1.0], Color::Red)
        .unwrap();
    figure
        .commands_mut()
        .writer_mut()
        .write_all(b"set output 'image.svg'\n")
        .unwrap();
    figure
        .commands_mut()
        .image(&Array2::from_row_major(2, 3, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]).unwrap())
        .unwrap();
    figure
        .commands_mut()
        .writer_mut()
        .write_all(b"unset output\n")
        .unwrap();
    assert!(figure.close().unwrap().success());
    let line = fs::read_to_string(directory.file("line.svg")).unwrap();
    let image = fs::read_to_string(directory.file("image.svg")).unwrap();
    assert!(line.contains("<svg") && line.contains("rgb(255,   0,   0)"));
    assert!(image.contains("<svg"));
    let plotted = image
        .split("id=\"gnuplot_plot_1\"")
        .nth(1)
        .unwrap()
        .split("</g>")
        .next()
        .unwrap();
    // With PNG support SVG embeds an image; without it all six matrix pixels
    // are explicit polygons inside the plotted-data group, not background axes.
    assert!(plotted.contains("<image ") || plotted.matches("<polygon ").count() == 6);
    if std::env::var_os("CIGLET_REQUIRE_GNUPLOT").is_some() {
        let mut default = Figure::open().unwrap();
        default
            .commands_mut()
            .writer_mut()
            .write_all(b"set terminal unknown\n")
            .unwrap();
        assert!(default.close().unwrap().success());
    }
}
