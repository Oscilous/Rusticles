extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;
const SAND_COLOR: u32 = 0xe4bc80;
const BACKGROUND_COLOR: u32 = 0x000000;
const CURSOR_COLOR: u32 = 0x00FF00;
fn main() {
    // Create a window with a specific resolution
    let mut window = Window::new(
        "Pixel Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4, // Scale factor
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");
    //Set FPS to ~60
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // Create a buffer to hold the pixel data
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Get the current mouse position
        if window.get_mouse_down(MouseButton::Left) {
            let mouse_pos: (f32, f32) = window.get_mouse_pos(MouseMode::Clamp).unwrap();
            // Get the coordinates of the mouse click
            let click_x: usize = mouse_pos.0 as usize;
            let click_y: usize = mouse_pos.1 as usize;

            // Set the pixel at the mouse click position to green
            buffer[click_x + click_y * WIDTH] = SAND_COLOR; // RGB value for green
        }

        update_physics(&mut buffer);

        // Render the buffer to the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn update_physics(buffer: &mut Vec<u32>) {
    for y in (1..(HEIGHT - 1)).rev() {
        for x in (1..(WIDTH - 1)).rev() {
            //Firstly sand physics
            if buffer[x + y * WIDTH] == SAND_COLOR {
                //If nothing is under, then gravity
                match buffer[x + (y + 1) * WIDTH] {
                    BACKGROUND_COLOR => {
                        buffer[x + (y + 1) * WIDTH] = SAND_COLOR;
                        buffer[x + y * WIDTH] = BACKGROUND_COLOR;
                    }
                    SAND_COLOR => {
                        if buffer[(x + 1) + y * WIDTH] == BACKGROUND_COLOR {
                            buffer[(x + 1) + y * WIDTH] = SAND_COLOR;
                            buffer[x + y * WIDTH] = BACKGROUND_COLOR;
                        } else if buffer[(x - 1) + y * WIDTH] == BACKGROUND_COLOR {
                            buffer[(x - 1) + y * WIDTH] = SAND_COLOR;
                            buffer[x + y * WIDTH] = BACKGROUND_COLOR;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
