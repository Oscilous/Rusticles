use crate::Particle;
use crate::Screen;

enum Position {
    RightSide,
    LeftSide,
    Middle,
}

pub fn gravity(screen: &mut Screen, x: usize, y: usize, particle: Particle) {
    if y != screen.height - 1 {
        screen.buffer[x + (y + 1) * screen.width] = particle.get_color();
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}

pub fn sink_solid(screen: &mut Screen, x: usize, y: usize) {
    if y != screen.height - 1 {
        screen.buffer[x + (y + 1) * screen.width] = Particle::Sand.get_color();
        screen.buffer[x + y * screen.width] = Particle::Water.get_color();
    }
}

pub fn solid_cascade(screen: &mut Screen, x: usize, y: usize) {
    match check_postion(screen, x) {
        Position::LeftSide => cascade_to_the_right(screen, x, y),
        Position::RightSide => cascade_to_the_left(screen, x, y),
        Position::Middle => {
            cascade_to_the_left(screen, x, y);
            cascade_to_the_right(screen, x, y);
        }
    }
}
fn check_postion(screen: &Screen, x: usize) -> Position {
    if x == 0 {
        Position::RightSide
    } else if x == (screen.width - 1) {
        Position::LeftSide
    } else {
        Position::Middle
    }
}

fn cascade_to_the_left(screen: &mut Screen, x: usize, y: usize) {
    if screen.buffer[(x + 1) + (y + 1) * screen.width] == Particle::Background.get_color()
        && screen.buffer[(x + 1) + y * screen.width] == Particle::Background.get_color()
    {
        println!("left");
        screen.buffer[(x + 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}

fn cascade_to_the_right(screen: &mut Screen, x: usize, y: usize) {
    if screen.buffer[(x - 1) + (y + 1) * screen.width] == Particle::Background.get_color()
        && screen.buffer[(x - 1) + y * screen.width] == Particle::Background.get_color()
    {
        println!("right");
        screen.buffer[(x - 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}

pub fn fluid_cascade(screen: &mut Screen, x: usize, y: usize) {
    match check_postion(screen, x) {
        Position::LeftSide => (),
        Position::RightSide => (),
        Position::Middle => {
            let mut index = 0;
            while (screen.buffer[x + index + y * screen.width] != Particle::Background.get_color()
                && screen.buffer[x - index + y * screen.width] != Particle::Background.get_color())
            {
                index += 1;
            }
        }
    }
}
