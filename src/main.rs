extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
mod physics;

struct Screen{
    WIDTH: usize,
    HEIGHT: usize,
    buffer: Vec<u32>
}

impl Screen{
    fn new(WIDTH: usize, HEIGHT: usize) -> Screen {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Calculate the third value

        Screen {
            WIDTH,
            HEIGHT,
            buffer,
        }
    }
}
struct Particle{
    const SAND_COLOR: u32 = 0xe4bc80;
    const BACKGROUND_COLOR: u32 = 0x000000;
}
enum Position {
    RightSide,
    LeftSide,
    Middle,
}

fn main() {
    // Create a window with a specific resolution
    let Screen: Screen = Screen::new(200, 100);
    let mut window = Window::new(
        "Pixel Renderer",
        Screen.WIDTH,
        Screen.HEIGHT,
        WindowOptions {
            scale: Scale::X4, // Scale factor
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");
    //Set FPS to ~60
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // Create a buffer to hold the pixel data
    window.HEIGHT
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
    for y in (0..(HEIGHT - 1)).rev() {
        for x in (0..(WIDTH - 1)).rev() {
            //Firstly sand physics
            if buffer[x + y * WIDTH] == SAND_COLOR {
                //If nothing is under, then gravity
                match buffer[x + (y + 1) * WIDTH] {
                    //If underneath a sand particle is nothing
                    BACKGROUND_COLOR => physics::gravity(buffer, x, y),
                    //If underneath a sand particle is another sand particle
                    SAND_COLOR => match check_for_cases(x, y) {
                        Position::RightSide => {
                            physics::right_corner_stacking(buffer, x, y, SAND_COLOR)
                        }
                        Position::LeftSide => {
                            physics::left_corner_stacking(buffer, x, y, SAND_COLOR)
                        }
                        Position::Middle => physics::middle_stacking(buffer, x, y, SAND_COLOR),
                    },
                    _ => (),
                }
            }
        }
    }
}

fn check_for_cases(x: usize, y: usize) -> Position {
    if x == 0 {
        Position::RightSide
    } else if x == WIDTH {
        Position::LeftSide
    } else {
        Position::Middle
    }
}
