use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum OutputType {
    Stdout,
    Stderr,
    File(PathBuf),
    Buffer(Arc<Mutex<Vec<u8>>>),
    Custom(Arc<dyn OutputWriter>),
}

impl std::fmt::Debug for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputType::Stdout => write!(f, "Stdout"),
            OutputType::Stderr => write!(f, "Stderr"),
            OutputType::File(path) => write!(f, "File({:?})", path),
            OutputType::Buffer(_) => write!(f, "Buffer"),
            OutputType::Custom(_) => write!(f, "Custom"),
        }
    }
}

pub trait OutputWriter: Send + Sync {
    fn write(&self, data: &[u8]) -> io::Result<()>;
    fn flush(&self) -> io::Result<()>;
}

pub struct Output {
    writer: Box<dyn OutputWriter>,
}

impl Output {
    pub fn new(output_type: OutputType) -> io::Result<Self> {
        let writer: Box<dyn OutputWriter> = match output_type {
            OutputType::Stdout => Box::new(StdoutWriter),
            OutputType::Stderr => Box::new(StderrWriter),
            OutputType::File(path) => Box::new(FileWriter::new(path)?),
            OutputType::Buffer(buffer) => Box::new(BufferWriter { buffer }),
            OutputType::Custom(writer) => Box::new(CustomWriterWrapper { writer }),
        };

        Ok(Self { writer })
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.writer.write(data)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.write(line.as_bytes())?;
        self.write(b"\n")?;
        self.flush()
    }
}

struct StdoutWriter;

impl OutputWriter for StdoutWriter {
    fn write(&self, data: &[u8]) -> io::Result<()> {
        io::stdout().write_all(data)
    }

    fn flush(&self) -> io::Result<()> {
        io::stdout().flush()
    }
}

struct StderrWriter;

impl OutputWriter for StderrWriter {
    fn write(&self, data: &[u8]) -> io::Result<()> {
        io::stderr().write_all(data)
    }

    fn flush(&self) -> io::Result<()> {
        io::stderr().flush()
    }
}

struct FileWriter {
    file: Arc<Mutex<std::fs::File>>,
}

impl FileWriter {
    fn new(path: PathBuf) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
        })
    }
}

impl OutputWriter for FileWriter {
    fn write(&self, data: &[u8]) -> io::Result<()> {
        if let Ok(mut file) = self.file.lock() {
            file.write_all(data)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to acquire file lock",
            ))
        }
    }

    fn flush(&self) -> io::Result<()> {
        if let Ok(mut file) = self.file.lock() {
            file.flush()
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to acquire file lock",
            ))
        }
    }
}

struct BufferWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl OutputWriter for BufferWriter {
    fn write(&self, data: &[u8]) -> io::Result<()> {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.extend_from_slice(data);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to acquire buffer lock",
            ))
        }
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

struct CustomWriterWrapper {
    writer: Arc<dyn OutputWriter>,
}

impl OutputWriter for CustomWriterWrapper {
    fn write(&self, data: &[u8]) -> io::Result<()> {
        self.writer.write(data)
    }

    fn flush(&self) -> io::Result<()> {
        self.writer.flush()
    }
}
