use core_graphics::{display::CGDisplay, display::CGPoint, display::CGRect};
use device_query::{DeviceQuery, DeviceState};
use std::{thread::sleep, time};

fn main() {
    // get mouse position
    let device_state = DeviceState::new();
    let mut last_mouse_pos = device_state.get_mouse().coords;

    // null display, use it set cursor position
    let null_display = CGDisplay::null_display();

    // init display id
    let last_display = get_mouse_in_which_display(&last_mouse_pos);

    loop {
        let mouse_pos = device_state.get_mouse().coords;

        // if new position not equal old position
        if mouse_pos != last_mouse_pos {
            // get which display is
            let now_display = get_mouse_in_which_display(&mouse_pos);

            // if cursor move to other display
            if now_display.id != last_display.id {
              // move cursor back
              null_display
                  .move_cursor_to_point(CGPoint {
                      x: last_mouse_pos.0 as f64,
                      y: last_mouse_pos.1 as f64,
                  })
                  .unwrap();
            } else {
              // update cursor position
              last_mouse_pos = mouse_pos;
            }
        }

        // sleep
        let sleep_time = time::Duration::from_millis(8);
        sleep(sleep_time);
    }
}

#[derive(Copy, Clone)]
struct Screen {
    id: u32,
    display: CGDisplay,
    bounds: CGRect,
}

// determine which screen the mouse is in, and return the screen id
fn get_mouse_in_which_display(pos: &(i32, i32)) -> Screen {
    let all_active_display = get_active_display();

    for &display in &all_active_display {
        let bounds = display.bounds;

        let pos_x = pos.0 as i32;
        let pos_y = pos.1 as i32;
        let display_x1 = bounds.origin.x as i32;
        let display_y1 = bounds.origin.y as i32;
        let display_x2 = bounds.size.width as i32 + display_x1;
        let display_y2 = bounds.size.height as i32 + display_y1;

        if pos_x >= display_x1 && pos_y >= display_y1 && pos_x <= display_x2 && pos_y <= display_y2
        {
            return display;
        }
    }

    all_active_display[0]
}

// get all active display
fn get_active_display() -> Vec<Screen> {
    let mut active_displays = Vec::new();

    // get all display id
    // build CGDisplay
    for id in CGDisplay::active_displays().unwrap() {
        let display = CGDisplay::new(id);
        let bounds = CGDisplay::bounds(&display);

        let screen = Screen {
            id,
            display,
            bounds,
        };

        active_displays.push(screen)
    }

    active_displays
}
