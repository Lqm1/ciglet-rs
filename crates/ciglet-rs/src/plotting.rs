//! Gnuplot commands on arbitrary writers, with owned native process integration.
use crate::{array::Array2, statistics::Scalar};
use std::fmt::Write as _;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Color {
    #[default]
    Blue,
    Red,
    Green,
    Black,
    Cyan,
    Yellow,
}
impl From<char> for Color {
    fn from(code: char) -> Self {
        match code {
            'r' => Self::Red,
            'g' => Self::Green,
            'k' => Self::Black,
            'c' => Self::Cyan,
            'y' => Self::Yellow,
            _ => Self::Blue,
        }
    }
}
impl Color {
    fn name(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Red => "red",
            Self::Green => "green",
            Self::Black => "black",
            Self::Cyan => "cyan",
            Self::Yellow => "yellow",
        }
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
fn bounded(length: usize) -> io::Result<()> {
    if length > i32::MAX as usize {
        return Err(invalid("plot dimension exceeds i32::MAX"));
    }
    Ok(())
}
fn nonfinite(value: f64) -> &'static str {
    match (value.is_nan(), value.is_sign_negative()) {
        (true, true) => "-nan",
        (true, false) => "nan",
        (false, true) => "-inf",
        (false, false) => "inf",
    }
}
fn coordinate(writer: &mut impl Write, value: f64) -> io::Result<()> {
    if value.is_finite() {
        write!(writer, "{value:.6}")
    } else {
        writer.write_all(nonfinite(value).as_bytes())
    }
}
fn image_value(writer: &mut impl Write, value: f64, scientific: &mut String) -> io::Result<()> {
    if !value.is_finite() {
        return write!(writer, "{:>5} ", nonfinite(value));
    }
    // Rust exponent formatting omits C's sign and two-digit minimum width.
    scientific.clear();
    write!(scientific, "{value:.3e}").expect("writing to a string");
    let (mantissa, exponent) = scientific.split_once('e').expect("scientific exponent");
    let exponent: i32 = exponent.parse().expect("numeric scientific exponent");
    write!(writer, "{mantissa}e{exponent:+03} ")
}

/// Locale-independent original command/data formatting. Nonfinite samples are
/// retained so callers can use Gnuplot's missing-point conventions.
#[derive(Debug)]
pub struct Commands<W> {
    writer: W,
}
impl<W> Commands<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
    /// Write arbitrary Gnuplot configuration, as with the source figure handle.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
    pub fn into_inner(self) -> W {
        self.writer
    }
}
impl<W: Write> Commands<W> {
    /// Extra x coordinates are ignored; missing x selects scalar-cast indices.
    /// Shapes are checked before output. I/O failures may leave a partial command.
    pub fn line<T: Scalar>(&mut self, x: Option<&[T]>, y: &[T], color: Color) -> io::Result<()> {
        bounded(y.len())?;
        if x.is_some_and(|values| values.len() < y.len()) {
            return Err(invalid("plot x buffer is shorter than y"));
        }
        writeln!(
            self.writer,
            "plot '-' with lines lc rgb \"{}\"",
            color.name()
        )?;
        for (index, &value) in y.iter().enumerate() {
            let position = x.map_or_else(|| T::from_usize(index).unwrap(), |values| values[index]);
            coordinate(&mut self.writer, position.into())?;
            self.writer.write_all(b" ")?;
            coordinate(&mut self.writer, value.into())?;
            self.writer.write_all(b"\n")?;
        }
        self.writer.write_all(b"e\n")?;
        self.writer.flush()
    }

    /// Row-major values with columns on x and rows on y, including empty shapes.
    pub fn image<T: Scalar>(&mut self, values: &Array2<T>) -> io::Result<()> {
        bounded(values.rows())?;
        bounded(values.columns())?;
        let mut scientific = String::new();
        scientific
            .try_reserve(32)
            .map_err(|_| io::Error::other("plot formatting allocation failed"))?;
        writeln!(
            self.writer,
            "set xrange [0.000000:{:.6}]",
            values.columns() as f64 - 1.0
        )?;
        writeln!(
            self.writer,
            "set yrange [0.000000:{:.6}]",
            values.rows() as f64 - 1.0
        )?;
        self.writer
            .write_all(b"set view map\nsplot '-' matrix with image\n")?;
        for row in 0..values.rows() {
            for &value in values.row(row).unwrap() {
                image_value(&mut self.writer, value.into(), &mut scientific)?;
            }
            self.writer.write_all(b"\n")?;
        }
        self.writer.write_all(b"e\n")?;
        self.writer.flush()
    }
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::Commands;
    use std::{
        ffi::OsStr,
        io::{self, BufWriter, Write},
        process::{Child, ChildStdin, Command, ExitStatus, Output, Stdio},
        sync::mpsc::{self, Receiver},
        thread,
    };

    /// A pipe and a background child waiter. Dropping closes stdin without
    /// killing persistent windows; the waiter reaps the main process on exit.
    /// Call `close` to observe flush and exit errors explicitly.
    pub struct Figure {
        commands: Option<Commands<BufWriter<ChildStdin>>>,
        completion: Receiver<io::Result<Output>>,
        id: u32,
    }
    impl Figure {
        pub fn open() -> io::Result<Self> {
            #[cfg(target_os = "windows")]
            let program = "gnuplot";
            #[cfg(not(target_os = "windows"))]
            let program = "gnuplot-qt";
            Self::open_with(program)
        }
        /// Launch the executable directly with `-p`, without a shell.
        pub fn open_with(program: impl AsRef<OsStr>) -> io::Result<Self> {
            Self::spawn(Command::new(program).arg("-p"))
        }
        /// Custom arguments/environment/output routing. This replaces the
        /// command's stdin with a pipe; persistence arguments are caller-owned.
        pub fn spawn(command: &mut Command) -> io::Result<Self> {
            let (handoff, child_receiver) = mpsc::channel::<Child>();
            let (completion_sender, completion) = mpsc::channel();
            // Establish reaping before creating the process. Spawn failures
            // close handoff, letting this detached worker exit without a child.
            thread::Builder::new()
                .name("ciglet-plot-wait".into())
                .spawn(move || {
                    if let Ok(child) = child_receiver.recv() {
                        let _ = completion_sender.send(child.wait_with_output());
                    }
                })?;
            let mut child = command.stdin(Stdio::piped()).spawn()?;
            let id = child.id();
            let Some(stdin) = child.stdin.take() else {
                let _ = child.kill();
                let _ = child.wait();
                return Err(io::Error::other("plot process stdin was not piped"));
            };
            if let Err(mpsc::SendError(mut child)) = handoff.send(child) {
                drop(stdin);
                let _ = child.kill();
                let _ = child.wait();
                return Err(io::Error::other("plot process waiter disconnected"));
            }
            Ok(Self {
                commands: Some(Commands::new(BufWriter::new(stdin))),
                completion,
                id,
            })
        }
        pub fn id(&self) -> u32 {
            self.id
        }
        pub fn commands_mut(&mut self) -> &mut Commands<BufWriter<ChildStdin>> {
            self.commands.as_mut().expect("open figure stream")
        }
        /// Flush, close stdin and wait for the main program. A caller-supplied
        /// command that waits for more than EOF can keep this call blocked.
        pub fn close(self) -> io::Result<ExitStatus> {
            self.close_with_output().map(|output| output.status)
        }
        /// Close and return any piped stdout/stderr. The worker drains both
        /// streams concurrently with command writes, avoiding full-pipe deadlock.
        /// Redirect large output to a file instead of capturing it in memory.
        pub fn close_with_output(mut self) -> io::Result<Output> {
            let flushed = self.commands_mut().writer_mut().flush();
            drop(self.commands.take());
            let status = self
                .completion
                .recv()
                .map_err(|_| io::Error::other("plot process waiter disconnected"))?;
            flushed?;
            status
        }
    }
    impl Drop for Figure {
        fn drop(&mut self) {
            // BufWriter's destructor attempts flushing; explicit close reports
            // errors. Closing this extracted pipe permits the worker's wait.
            drop(self.commands.take());
        }
    }
}
#[cfg(not(target_family = "wasm"))]
pub use native::Figure;
