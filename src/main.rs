use core_graphics::{display::CGDisplay, display::CGPoint, display::CGRect};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread::sleep, time};

fn main() {
    let device_state = DeviceState::new();
    let mut last_mouse_pos = device_state.get_mouse().coords;

    // use it set cursor position
    let null_display = CGDisplay::null_display();

    // init last_display
    let mut last_display = get_mouse_in_which_display(&last_mouse_pos);

    // set which key could move cursor to next display
    let switch_key = vec![Keycode::Meta, Keycode::Grave];

    let mut _is_key_down = false;
    // prev press key
    let mut last_key: Vec<Keycode> = Vec::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let mouse_pos = device_state.get_mouse().coords;

        // deal with mouse
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

        // deal with keyboard
        // if press some key
        if !keys.is_empty() {
            // if new keys length
            if keys.len() > last_key.len() {
                _is_key_down = true;
            } else if _is_key_down {
                // _is_key_down only equal true once.
                _is_key_down = false;
            }
        } else {
            // end set _is_key_down to false
            _is_key_down = false;
        }

        if keys == switch_key && _is_key_down {
            // get new mouse position
            let next_display = get_next_display(last_display);
            let new_pos = get_new_position(&mouse_pos, &last_display, &next_display);

            // set cursor
            null_display
                .move_cursor_to_point(CGPoint {
                    x: new_pos.0 as f64,
                    y: new_pos.1 as f64,
                })
                .unwrap();

            // set last_display and last_mouse_pos to new value
            last_display = next_display;
            last_mouse_pos = new_pos;
        }

        // final set last_key
        if keys != last_key {
            last_key = keys;
        }

        // sleep thread 20 ms
        let sleep_time = time::Duration::from_millis(20);
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

//get next display,if current is end display,return first.
fn get_next_display(current_display: Screen) -> Screen {
    let all_active_display = get_active_display();

    let mut flag = false;

    for &display in &all_active_display {
        if flag {
            return display;
        }
        if display.id == current_display.id {
            flag = true
        }
    }

    all_active_display[0]
}

// get new position in next display
fn get_new_position(
    pos: &(i32, i32),
    current_display: &Screen,
    next_display: &Screen,
) -> (i32, i32) {
    let pos_x = pos.0 as f64;
    let pos_y = pos.1 as f64;
    let display_x1 = current_display.bounds.origin.x;
    let display_y1 = current_display.bounds.origin.y;

    let x_scale = (pos_x - display_x1) / current_display.bounds.size.width;
    let y_scale = (pos_y - display_y1) / current_display.bounds.size.height;

    let result_x = x_scale * next_display.bounds.size.width + next_display.bounds.origin.x;
    let result_y = y_scale * next_display.bounds.size.height + next_display.bounds.origin.y;

    (result_x as i32, result_y as i32)
}
