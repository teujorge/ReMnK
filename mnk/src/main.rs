mod utils;

use rdev::{listen, Event, EventType};
use serde_json::json;
use std::{
    io::{BufWriter, Write},
    time::UNIX_EPOCH,
};
use utils::WriteData;

fn main() {
    let mut writer = utils::create_ipc_stream();

    if let Err(error) = listen(move |event| {
        let formatted_event = format_event(&event);
        handle_and_send_event(&mut writer, &formatted_event);
    }) {
        eprintln!("Error listening to events: {:?}", error);
    }
}

fn format_event(event: &Event) -> String {
    // println!("Event: {:?}", event);

    let time_since_epoch = event
        .time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    let json = match event.event_type {
        EventType::KeyPress(key) => {
            json!({ "keyPress": format!("{:?}", key) })
        }
        EventType::KeyRelease(key) => {
            json!({ "keyRelease": format!("{:?}", key) })
        }
        EventType::ButtonPress(button) => {
            json!({ "buttonPress": format!("{:?}", button) })
        }
        EventType::ButtonRelease(button) => {
            json!({ "buttonRelease": format!("{:?}", button) })
        }
        EventType::MouseMove { x, y } => {
            json!({ "mouseMove": { "x": x, "y": y, "time": time_since_epoch }
            })
        }
        _ => {
            return "".to_string();
        } // Ignore other events
    };
    return json.to_string() + "\n";
}

fn handle_and_send_event<W: WriteData>(writer: &mut BufWriter<W>, message: &str) {
    if message.is_empty() {
        return;
    }

    if let Err(e) = writer.write_all(message.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
        return;
    }
    if let Err(e) = writer.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}
