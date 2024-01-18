// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import necessary modules
mod controller;
mod ipc;

// Import necessary crates
use std::process::{Child, Command};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Manager;

fn main() {
    // Create a channel for thread communication
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Create a mutex for the previous mouse event
    let mutex_prev_mouse_event_data =
        Arc::new(Mutex::new(controller::MouseEventData::new(0.0, 0.0, 0.0)));

    // Run IPC server in a separate thread
    thread::spawn(move || {
        println!("IPC server starting...");
        ipc::handle_ipc(tx);
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
        .setup(move |app| {
            let handle_for_middleman = app.handle();
            let handle_for_no_mouse_event = app.handle();

            let mutex_prev_mouse_event_data_clone_for_middleman =
                mutex_prev_mouse_event_data.clone();
            let mutex_prev_mouse_event_data_clone_for_no_mouse_event =
                mutex_prev_mouse_event_data.clone();

            println!("Tauri setup -> spawn mnk middleman thread");
            // Use the `rx` here in the main thread to receive messages
            std::thread::spawn(move || {
                for received in rx {
                    // println!("tauri to send data to FE: {}", received);

                    // Send the received message to the FE as mnk-event
                    handle_for_middleman
                        .emit_all("mnk-event", &received)
                        .expect("Failed to emit mnk-event");

                    // Virtualize the controller based on the received message
                    let controller_event = controller::virtualize_controller(
                        &received,
                        &mutex_prev_mouse_event_data_clone_for_middleman,
                    );

                    // Send the received message to the FE as controller-event
                    handle_for_middleman
                        .emit_all("controller-event", &controller_event)
                        .expect("Failed to emit controller-event");
                }
            });

            println!("Tauri setup -> spawn no mouse events thread");
            // Spawn a thread to handle no mouse events
            std::thread::spawn({
                let mutex_prev_mouse_event_data_clone =
                    mutex_prev_mouse_event_data_clone_for_no_mouse_event.clone();
                move || {
                    controller::handle_no_mouse_events(
                        &handle_for_no_mouse_event,
                        mutex_prev_mouse_event_data_clone,
                    );
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

// Start the listener binary (mnk)
fn start_mnk_listener() -> Child {
    let listener_path = "../mnk/target/release/mnk";
    return Command::new(listener_path)
        .spawn()
        .expect("Failed to start listener");
}
