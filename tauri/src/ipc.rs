use std::io::Read;
use std::sync::mpsc::Sender;

#[cfg(windows)]
use named_pipe::PipeOptions;
#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};

pub const IPC_PIPE_NAME: &str = r"\\.\pipe\remnk";
pub const IPC_SOCKET_PATH: &str = "/tmp/remnk.socket";

pub fn handle_ipc(tx: Sender<String>) {
    #[cfg(unix)]
    {
        handle_ipc_unix(tx);
    }
    #[cfg(windows)]
    {
        handle_ipc_windows(tx);
    }
}

#[cfg(unix)]
fn handle_ipc_unix(tx: Sender<String>) {
    let listener = UnixListener::bind(IPC_SOCKET_PATH).expect("Failed to bind to IPC socket");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    match line {
                        Ok(key_str) => {
                            println!("Received message from Unix IPC: {}", key_str);

                            if key_str.is_empty() {
                                continue;
                            }

                            tx.send(key_str)
                                .expect("Failed to send key message to Tauri");
                        }
                        Err(e) => {
                            eprintln!("Failed to read from IPC stream: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }
}

#[cfg(windows)]
fn handle_ipc_windows(tx: Sender<String>) {
    let connection = PipeOptions::new(IPC_PIPE_NAME)
        .first(true)
        .single()
        .expect("Failed to create named pipe server");

    let mut server = connection
        .wait()
        .expect("Failed to connect to named pipe server");

    let mut buffer = [0; 1024];

    loop {
        let bytes_read = server
            .read(&mut buffer)
            .expect("Failed to read from named pipe server");

        if bytes_read == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        // println!("Received message from Windows IPC: {}", message);

        if message.is_empty() {
            continue;
        }

        tx.send(message.to_string())
            .expect("Failed to send key message to Tauri");
    }
}
