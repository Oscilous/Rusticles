extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
mod physics;

struct Screen {
    WIDTH: usize,
    HEIGHT: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl Screen {
    fn new(WIDTH: usize, HEIGHT: usize) -> Screen {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Calculate the third value
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
        Screen {
            WIDTH,
            HEIGHT,
            buffer,
            window,
        }
    }
}
enum Particle {
    Sand,
    Background,
}

impl Particle {
    fn get_color(&self) -> u32 {
        match *self {
            Particle::Sand => 0xe4bc80,
            Particle::Background => 0x000000,
        }
    }
}
enum Position {
    RightSide,
    LeftSide,
    Middle,
}

fn main() {
    // Create a window with a specific resolution
    let mut Screen: Screen = Screen::new(200, 100);
    while Screen.window.is_open() && !Screen.window.is_key_down(Key::Escape) {
        // Get the current mouse position
        if Screen.window.get_mouse_down(MouseButton::Left) {
            let mouse_pos: (f32, f32) = Screen.window.get_mouse_pos(MouseMode::Clamp).unwrap();
            // Get the coordinates of the mouse click
            let click_x: usize = mouse_pos.0 as usize;
            let click_y: usize = mouse_pos.1 as usize;

            // Set the pixel at the mouse click position to green
            Screen.buffer[click_x + click_y * Screen.WIDTH] = Particle::Sand.get_color();
            // RGB value for green
        }

        update_physics(&mut Screen);

        // Render the buffer to the window
        Screen
            .window
            .update_with_buffer(&Screen.buffer, Screen.WIDTH, Screen.HEIGHT)
            .unwrap();
    }
}

fn update_physics(Screen: &mut Screen) {
    for y in (0..(Screen.HEIGHT - 1)).rev() {
        for x in (0..(Screen.WIDTH - 1)).rev() {
            //Firstly sand physics
            if Screen.buffer[x + y * Screen.WIDTH] == Particle::Sand.get_color() {
                //If nothing is under, then gravity
                match Screen.buffer[x + (y + 1) * Screen.WIDTH] {
                    //If underneath a sand particle is nothing
                    BACKGROUND_COLOR => physics::gravity(Screen, x, y),
                    //If underneath a sand particle is another sand particle
                    SAND_COLOR => match check_for_cases(&Screen, x, y) {
                        Position::RightSide => physics::right_corner_stacking(Screen, x, y),
                        Position::LeftSide => physics::left_corner_stacking(Screen, x, y),
                        Position::Middle => physics::middle_stacking(Screen, x, y),
                    },
                    _ => (),
                }
            }
        }
    }
}

fn check_for_cases(Screen: &Screen, x: usize, y: usize) -> Position {
    if x == 0 {
        Position::RightSide
    } else if x == Screen.WIDTH {
        Position::LeftSide
    } else {
        Position::Middle
    }
}
