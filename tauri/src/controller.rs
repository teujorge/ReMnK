use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct MouseEventData {
    x: f64,
    y: f64,
    timestamp: Instant,
}

impl MouseEventData {
    pub fn new(x: f64, y: f64) -> Self {
        MouseEventData {
            x,
            y,
            timestamp: Instant::now(),
        }
    }
}

// Virtualize controller based on mnk input
// returns a string or list of string (size 2)
pub fn virtualize_controller(
    mnk_json: &String,
    mutex_prev_mouse_event_data: &Arc<Mutex<MouseEventData>>,
) -> String {
    // println!("virtualize_controller: {}", mnk_json);

    match serde_json::from_str::<serde_json::Value>(mnk_json) {
        Ok(mnk_event) => {
            // println!("mnk_event: {}", mnk_event);

            let key_press = mnk_event["keyPress"].as_str();
            let key_release = mnk_event["keyRelease"].as_str();

            let button_press = mnk_event["buttonPress"].as_str();
            let button_release = mnk_event["buttonRelease"].as_str();

            let mouse_move = mnk_event["mouseMove"].as_object();
            let wheel = mnk_event["wheel"].as_object();

            if key_press != None {
                match key_press.unwrap() {
                    "KeyW" => return "press_l3_up".to_owned(),
                    "KeyA" => return "press_l3_left".to_owned(),
                    "KeyS" => return "press_l3_down".to_owned(),
                    "KeyD" => return "press_l3_right".to_owned(),
                    "KeyE" => return "press_start".to_owned(),
                    "KeyQ" => return "press_select".to_owned(),
                    "KeyR" => return "press_square".to_owned(),
                    "KeyC" => return "press_circle".to_owned(),
                    "KeyF" => return "press_triangle".to_owned(),
                    "KeyV" => return "press_r1".to_owned(),
                    "Space" => return "press_cross".to_owned(),
                    "ShiftLeft" => return "press_l3_in".to_owned(),
                    "Num1" => return "press_pad_up".to_owned(),
                    "Num2" => return "press_pad_down".to_owned(),
                    "Num3" => return "press_pad_left".to_owned(),
                    "Num4" => return "press_pad_right".to_owned(),
                    _ => return "".to_owned(),
                }
            }

            if key_release != None {
                match key_release.unwrap() {
                    "KeyW" => return "release_l3_up".to_owned(),
                    "KeyA" => return "release_l3_left".to_owned(),
                    "KeyS" => return "release_l3_down".to_owned(),
                    "KeyD" => return "release_l3_right".to_owned(),
                    "KeyE" => return "release_start".to_owned(),
                    "KeyQ" => return "release_select".to_owned(),
                    "KeyR" => return "release_square".to_owned(),
                    "KeyC" => return "release_circle".to_owned(),
                    "KeyF" => return "release_triangle".to_owned(),
                    "KeyV" => return "release_r1".to_owned(),
                    "Space" => return "release_cross".to_owned(),
                    "ShiftLeft" => return "release_l3_in".to_owned(),
                    "Num1" => return "release_pad_up".to_owned(),
                    "Num2" => return "release_pad_down".to_owned(),
                    "Num3" => return "release_pad_left".to_owned(),
                    "Num4" => return "release_pad_right".to_owned(),
                    _ => return "".to_owned(),
                }
            }

            if button_press != None {
                match button_press.unwrap() {
                    "Left" => return "press_r2".to_owned(),
                    "Right" => return "press_l2".to_owned(),
                    "Middle" => return "press_r3_in".to_owned(),
                    _ => return "".to_owned(),
                }
            }

            if button_release != None {
                match button_release.unwrap() {
                    "Left" => return "release_r2".to_owned(),
                    "Right" => return "release_l2".to_owned(),
                    "Middle" => return "release_r3_in".to_owned(),
                    _ => return "".to_owned(),
                }
            }

            if mouse_move != None {
                let mouse_move = mouse_move.unwrap();
                let x = mouse_move["x"].as_f64().unwrap();
                let y = mouse_move["y"].as_f64().unwrap();

                // Lock the mutex to safely access and update the shared state
                let mut prev_mouse_event = mutex_prev_mouse_event_data.lock().unwrap();

                let new_mouse_event = MouseEventData::new(x, y);

                // Calculate the controller movement based on the current and previous mouse data
                let controller_command =
                    translate_mouse_to_controller(&new_mouse_event, &prev_mouse_event);

                // Update the previous mouse event data
                *prev_mouse_event = new_mouse_event;

                // Controller command logic...
                return controller_command.to_owned();
            }

            if wheel != None {
                let wheel = wheel.unwrap();
                let dx = wheel["dx"].as_i64().unwrap();
                let dy = wheel["dy"].as_i64().unwrap();
                if dx > 0 {
                    return "move_wheel_right".to_owned();
                } else if dx < 0 {
                    return "move_wheel_left".to_owned();
                } else if dy > 0 {
                    return "move_wheel_down".to_owned();
                } else if dy < 0 {
                    return "move_wheel_up".to_owned();
                } else if dx == 0 {
                    return "release_wheel_x".to_owned();
                } else if dy == 0 {
                    return "release_wheel_y".to_owned();
                }
            }

            return "".to_owned();
        }
        Err(e) => {
            // Handle JSON parsing error
            eprintln!("JSON parsing error: {}", e);
            return "invalid_json".to_owned();
        }
    }
}

fn translate_mouse_to_controller(current: &MouseEventData, previous: &MouseEventData) -> String {
    let delta_x = current.x - previous.x;
    let delta_y = current.y - previous.y;
    let time_elapsed = current
        .timestamp
        .duration_since(previous.timestamp)
        .as_secs_f64();

    if time_elapsed <= 0.0 {
        // {x: 0, y: 0}
        return "r3_0_0".to_owned();
    }
    // adjust the scaling factor as needed ...

    let delta_magnitude = (delta_x * delta_x + delta_y * delta_y).sqrt();
    let speed_scalar = delta_magnitude / time_elapsed;
    let direction_vector = vec![delta_x / delta_magnitude, delta_y / delta_magnitude];

    // println!(
    //     "translate_mouse_to_controller: {}  |  {} ms",
    //     format!(
    //         "r3 | x%{} | y%{}",
    //         (direction_vector[0] * speed_scalar).floor(),
    //         (direction_vector[1] * speed_scalar).floor(),
    //     ),
    //     (time_elapsed * 1000.0).floor()
    // );

    return format!(
        "r3_{}_{}",
        direction_vector[0] * speed_scalar,
        direction_vector[1] * speed_scalar,
    );
}
