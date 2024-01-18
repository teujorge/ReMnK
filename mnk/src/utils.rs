use std::io::{BufWriter, Write};

#[cfg(windows)]
use named_pipe::PipeClient;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

pub trait WriteData: Write {
    fn write_data(&mut self, data: &[u8]) -> std::io::Result<()>;
    fn flush_data(&mut self) -> std::io::Result<()>;
}

#[cfg(unix)]
impl WriteData for UnixStream {
    fn write_data(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.write_all(data)
    }
    fn flush_data(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}

#[cfg(windows)]
impl WriteData for PipeClient {
    fn write_data(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.write_all(data)
    }
    fn flush_data(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}

impl WriteData for Box<dyn WriteData> {
    fn write_data(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.write_all(data)
    }
    fn flush_data(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}

pub fn create_ipc_stream() -> BufWriter<Box<dyn WriteData>> {
    #[cfg(unix)]
    let stream: Box<dyn WriteData> =
        Box::new(UnixStream::connect("/tmp/remnk.socket").expect("Failed to connect to IPC"));

    #[cfg(windows)]
    let stream: Box<dyn WriteData> =
        Box::new(PipeClient::connect(r"\\.\pipe\remnk").expect("Failed to connect to IPC"));

    return BufWriter::new(stream);
}
