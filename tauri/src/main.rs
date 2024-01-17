// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import necessary modules
mod controller;

// Import necessary crates
use crate::controller::{virtualize_controller, MouseEventData};
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::{Child, Command};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use tauri::Manager;

// #[cfg(windows)]
// use named_pipe::PipeServer;
// #[cfg(unix)]
// use uds::UnixListener;

fn main() {
    // Create a channel for thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Delete the existing socket file if it exists
    let socket_path = "/tmp/remnk.socket";
    let _ = fs::remove_file(socket_path);

    // Create a mutex for the previous mouse event
    let mutex_prev_mouse_event_data = Arc::new(Mutex::new(MouseEventData::new(0.0, 0.0)));

    // Run IPC server in a separate thread
    thread::spawn(move || {
        // #[cfg(windows)]
        //   let listener = /* Setup named pipe server */;
        #[cfg(unix)]
        let listener =
            UnixListener::bind("/tmp/remnk.socket").expect("Failed to bind to IPC socket");

        println!("IPC server started");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection established");
                    let tx = tx.clone();
                    thread::spawn(move || handle_client(stream, tx));
                }
                Err(e) => {
                    eprintln!("Error in IPC server: {}", e);
                }
            }
        }
    });

    // Start the listener binary (mnk)
    let mnk_process = Arc::new(Mutex::new(start_mnk_listener()));

    // Clone Arc for use in the closure
    let mnk_process_clone = mnk_process.clone();

    // Run Tauri application on the main thread
    println!("Tauri thread (main) started");

    tauri::Builder::default()
        .manage(mnk_process)
        // Setup Tauri app to receive messages from the listener process and send them to FE
        .setup(|app| {
            let handle = app.handle();

            println!("Tauri setup -> spawn thread");
            // Use the `rx` here in the main thread to receive messages
            std::thread::spawn(move || {
                for received in rx {
                    // println!("tauri to send data to FE: {}", received);

                    // Send the received message to the FE as mnk-event
                    handle
                        .emit_all("mnk-event", &received)
                        .expect("Failed to emit mnk-event");

                    // Send the received message to the FE as controller-event
                    let controller_event =
                        virtualize_controller(&received, &mutex_prev_mouse_event_data);
                    handle
                        .emit_all("controller-event", &controller_event)
                        .expect("Failed to emit controller-event");
                }
            });

            Ok(())
        })
        .on_window_event(move |event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                println!("Window Event | Close Requested");
                let mut mnk_process = mnk_process_clone.lock().unwrap();
                if let Err(e) = mnk_process.kill() {
                    eprintln!("Failed to kill mnk listener process: {}", e);
                    api.prevent_close();
                    println!("Window Event | Api | Prevent Close");
                }
            }
            _ => {}
        })
        // Run the Tauri app
        .run(tauri::generate_context!())
        // Handle the Tauri app's error
        .expect("error while running tauri application");
}

// Handle a new client connection
fn handle_client(stream: UnixStream, tx: Sender<String>) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(key_str) => {
                // println!("handle_client Received key: {}", key_str);
                tx.send(key_str)
                    .expect("Failed to send key message to Tauri");
            }
            Err(e) => {
                eprintln!("Failed to read from IPC stream: {}", e);
                break;
            }
        }
    }
}

// Start the listener binary (mnk)
fn start_mnk_listener() -> Child {
    let listener_path = "../mnk/target/release/mnk";
    return Command::new(listener_path)
        .spawn()
        .expect("Failed to start listener");
}
