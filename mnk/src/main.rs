use rdev::{listen, Event, EventType};
use serde_json::json;
use std::io::{BufWriter, Write};
use std::os::unix::net::UnixStream;

fn main() {
    let stream = UnixStream::connect("/tmp/remnk.socket").expect("Failed to connect to IPC socket");
    let mut writer = BufWriter::new(stream);

    if let Err(error) = listen(move |event| {
        let formatted_event = format_event(&event);
        handle_and_send_event(&mut writer, &formatted_event);
    }) {
        eprintln!("Error listening to events: {:?}", error);
    }
}

fn format_event(event: &Event) -> String {
    println!("Event: {:?}", event);

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
            json!({ "mouseMove": { "x": x,       "y": y        }
            })
        }
        _ => {
            return "".to_string();
        } // Ignore other events
    };
    return json.to_string() + "\n";
}

fn handle_and_send_event(writer: &mut BufWriter<UnixStream>, message: &str) {
    if message == "" {
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
