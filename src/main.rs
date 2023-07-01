extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
mod physics;

pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let buffer: Vec<u32> = vec![0; width * height]; // Calculate the third value
        let mut window = Window::new(
            "Pixel Renderer",
            width,
            height,
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
            width,
            height,
            buffer,
            window,
        }
    }
    fn as_particle(&self, index: usize) -> Particle {
        match self.buffer[index] {
            0x000000 => Particle::Background,
            0xe4bc80 => Particle::Sand,
            _ => panic!("Unknown particle"),
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

fn main() {
    // Create a window with a specific resolution
    let mut screen: Screen = Screen::new(200, 100);
    while screen.window.is_open() && !screen.window.is_key_down(Key::Escape) {
        // Get the current mouse position
        if screen.window.get_mouse_down(MouseButton::Left) {
            let mouse_pos: (f32, f32) = screen.window.get_mouse_pos(MouseMode::Clamp).unwrap();
            // Get the coordinates of the mouse click
            let click_x: usize = mouse_pos.0 as usize;
            let click_y: usize = mouse_pos.1 as usize;
            println!("x: {click_x}, y: {click_y}");
            // Set the pixel at the mouse click position to green
            screen.buffer[click_x + click_y * screen.width] = Particle::Sand.get_color();
            // RGB value for green
        }

        update_physics(&mut screen);

        // Render the buffer to the window
        screen
            .window
            .update_with_buffer(&screen.buffer, screen.width, screen.height)
            .unwrap();
    }
}

fn update_physics(screen: &mut Screen) {
    for y in (0..(screen.height - 1)).rev() {
        for x in (0..(screen.width)).rev() {
            //Firstly sand physics
            if screen.buffer[x + y * screen.width] == Particle::Sand.get_color() {
                //If nothing is under, then gravity
                match screen.as_particle(x + (y + 1) * screen.width) {
                    //If underneath a sand particle is nothing
                    Particle::Background => physics::gravity(screen, x, y),
                    //If underneath a sand particle is another sand particle
                    Particle::Sand => physics::cascade(screen, x, y),
                }
            }
        }
    }
}
