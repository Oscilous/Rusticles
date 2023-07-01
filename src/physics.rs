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
        Position::LeftSide => {
            let mut negative_index = 0;
            while screen.buffer[x - negative_index + (y + 1) * screen.width]
                != Particle::Background.get_color()
                && x - negative_index != 0
            {
                negative_index += 1;
            }
            screen.buffer[x - negative_index + y * screen.width] = Particle::Water.get_color();
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
        }
        Position::RightSide => {
            let mut positive_index = 0;
            while screen.buffer[x + positive_index + (y + 1) * screen.width]
                != Particle::Background.get_color()
                && x + positive_index != screen.width - 1
            {
                positive_index += 1;
            }
            screen.buffer[x + positive_index + y * screen.width] = Particle::Water.get_color();
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
        }
        Position::Middle => {
            let target_index = check_for_closest_side(screen, x, y);
            screen.buffer[target_index + y * screen.width] = Particle::Water.get_color();
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
        }
    }
}

fn check_for_closest_side(screen: &mut Screen, x: usize, y: usize) -> usize {
    let mut at_edge: bool = false;
    let mut positive_index = 0;
    while screen.buffer[x + positive_index + (y + 1) * screen.width]
        != Particle::Background.get_color()
    {
        if x + positive_index == screen.width - 1 {
            at_edge = true;
            break;
        }
        positive_index += 1;
    }

    let mut negative_index = 0;
    while screen.buffer[x - negative_index + (y + 1) * screen.width]
        != Particle::Background.get_color()
    {
        if x - negative_index == 0 {
            at_edge = true;
            break;
        }
        negative_index += 1;
    }

    if positive_index < negative_index {
        x + positive_index
    } else if at_edge == true {
        x
    } else {
        x - negative_index
    }
}
