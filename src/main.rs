use core_graphics::{display::CGDisplay, display::CGPoint};
use device_query::{DeviceQuery, DeviceState};
use std::{thread::sleep, time};

fn main() {
    // get mouse position
    let device_state = DeviceState::new();
    let last_mouse_pos = device_state.get_mouse().coords;
    let cg_display = CGDisplay::null_display();

    loop {
        let mouse_pos = device_state.get_mouse().coords;

        // if new position not equal old position
        if mouse_pos != last_mouse_pos {
            // move cursor
            cg_display
                .move_cursor_to_point(CGPoint {
                    x: last_mouse_pos.0 as f64,
                    y: last_mouse_pos.1 as f64,
                })
                .unwrap();
        }

        // sleep
        let sleep_time = time::Duration::from_millis(8);
        sleep(sleep_time);
    }
}
