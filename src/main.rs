extern crate minifb;

use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use std::thread::sleep;
use std::time;
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
            0x74ccf4 => Particle::Water,
            _ => panic!("Unknown particle"),
        }
    }
    fn update_window(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
    fn update_physics(&mut self) {
        let mut sand_amount = 0;
        for y in (0..(self.height)).rev() {
            for x in (0..(self.width)).rev() {
                //Firstly sand physics
                match self.as_particle(x + y * self.width) {
                    Particle::Sand => {
                        sand_amount += 1;
                        //If nothing is under, then gravity
                        if y != self.height - 1 {
                            match self.as_particle(x + (y + 1) * self.width) {
                                //If underneath a sand particle is nothing
                                Particle::Background => {
                                    physics::gravity(self, x, y, Particle::Sand)
                                }
                                //If underneath a sand particle is another sand particle
                                Particle::Sand => physics::solid_cascade(self, x, y),
                                Particle::Water => physics::sink_solid(self, x, y),
                            }
                        }
                    }
                    Particle::Water => {
                        //If nothing is under, then gravity
                        if y != self.height - 1 {
                            match self.as_particle(x + (y + 1) * self.width) {
                                //If underneath a sand particle is nothing
                                Particle::Background => {
                                    physics::gravity(self, x, y, Particle::Water)
                                }
                                //If underneath a sand particle is another sand particle
                                Particle::Sand => physics::solid_cascade(self, x, y),
                                Particle::Water => physics::fluid_cascade(self, x, y),
                            }
                        }
                    }
                    Particle::Background => (),
                }
            }
        }
        println!("Sand: {sand_amount}");
    }
}
pub enum Particle {
    Sand,
    Water,
    Background,
}

impl Particle {
    fn get_color(&self) -> u32 {
        match *self {
            Particle::Sand => 0xe4bc80,
            Particle::Water => 0x74ccf4,
            Particle::Background => 0x000000,
        }
    }
}

fn main() {
    // Create a window with a specific resolution
    let mut screen: Screen = Screen::new(200, 100);
    let mut selected_particle: Particle = Particle::Sand;
    screen.update_window();
    while screen.window.is_open() && !screen.window.is_key_down(Key::Escape) {
        // Get the current mouse position
        if screen.window.get_mouse_down(MouseButton::Left) {
            let mouse_pos: (f32, f32) = screen.window.get_mouse_pos(MouseMode::Clamp).unwrap();
            // Get the coordinates of the mouse click
            let click_x: usize = mouse_pos.0 as usize;
            let click_y: usize = mouse_pos.1 as usize;
            println!("x: {click_x}, y: {click_y}");
            // Set the pixel at the mouse click position to green
            screen.buffer[click_x + click_y * screen.width] = selected_particle.get_color();
            // RGB value for green
        }
        if screen.window.is_key_down(Key::S) {
            selected_particle = Particle::Sand;
        }
        if screen.window.is_key_down(Key::W) {
            selected_particle = Particle::Water;
        }
        screen.update_physics();
        screen.update_window();
        sleep(time::Duration::from_millis(1));
    }
}
