use crate::Particle;
use crate::Screen;

enum Position {
    RightSide,
    LeftSide,
    Middle,
}

pub fn gravity(screen: &mut Screen, x: usize, y: usize, particle: Particle) {
    screen.buffer[x + (y + 1) * screen.width] = particle.get_color();
    screen.buffer[x + y * screen.width] = Particle::Background.get_color();
}

pub fn sink_solid(screen: &mut Screen, x: usize, y: usize) {
    screen.buffer[x + (y + 1) * screen.width] = Particle::Sand.get_color();
    screen.buffer[x + y * screen.width] = Particle::Water.get_color();
}

pub fn solid_cascade(screen: &mut Screen, x: usize, y: usize) {
    match check_postion(screen, x) {
        Position::LeftSide => cascade_to_the_right(screen, x, y),
        Position::RightSide => cascade_to_the_left(screen, x, y),
        Position::Middle => cascade_middle(screen, x, y),
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
    match screen
        .as_particle((x + 1) + (y + 1) * screen.width)
        .is_water_or_background()
        && screen
            .as_particle((x + 1) + y * screen.width)
            .is_water_or_background()
    {
        true => {
            screen.buffer[x + y * screen.width] = screen.buffer[(x + 1) + (y + 1) * screen.width];
            screen.buffer[(x + 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
        }
        false => (),
    };
}

fn cascade_to_the_right(screen: &mut Screen, x: usize, y: usize) {
    match screen
        .as_particle((x - 1) + (y + 1) * screen.width)
        .is_water_or_background()
        && screen
            .as_particle((x - 1) + y * screen.width)
            .is_water_or_background()
    {
        true => {
            screen.buffer[x + y * screen.width] = screen.buffer[(x - 1) + (y + 1) * screen.width];
            screen.buffer[(x - 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
        }
        false => (),
    };
}

fn cascade_middle(screen: &mut Screen, x: usize, y: usize) {
    if screen
        .as_particle((x + 1) + (y + 1) * screen.width)
        .is_water_or_background()
        && screen
            .as_particle((x + 1) + y * screen.width)
            .is_water_or_background()
    {
        screen.buffer[x + y * screen.width] = screen.buffer[(x + 1) + (y + 1) * screen.width];
        screen.buffer[(x + 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
    } else if screen
        .as_particle((x - 1) + (y + 1) * screen.width)
        .is_water_or_background()
        && screen
            .as_particle((x - 1) + y * screen.width)
            .is_water_or_background()
    {
        screen.buffer[x + y * screen.width] = screen.buffer[(x - 1) + (y + 1) * screen.width];
        screen.buffer[(x - 1) + (y + 1) * screen.width] = Particle::Sand.get_color();
    }
}

pub fn fluid_level(screen: &mut Screen, x: usize, y: usize) {
    let mut positive_index = 0;
    loop {
        positive_index += 1;
        if screen.buffer[x + positive_index + (y + 1) * screen.width] != Particle::Water.get_color()
        {
            break;
        }
    }

    let mut negative_index = 0;
    loop {
        negative_index += 1;
        if screen.buffer[x - negative_index + (y + 1) * screen.width] != Particle::Water.get_color()
        {
            break;
        }
    }
    if positive_index < negative_index {
        if screen.buffer[x + positive_index + (y + 1) * screen.width]
            == Particle::Background.get_color()
        {
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
            screen.buffer[x + positive_index + (y + 1) * screen.width] =
                Particle::Water.get_color();
        } else if screen.buffer[x - negative_index + (y + 1) * screen.width]
            == Particle::Background.get_color()
        {
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
            screen.buffer[x - negative_index + (y + 1) * screen.width] =
                Particle::Water.get_color();
        }
    } else if positive_index >= negative_index {
        if screen.buffer[x - negative_index + (y + 1) * screen.width]
            == Particle::Background.get_color()
        {
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
            screen.buffer[x - negative_index + (y + 1) * screen.width] =
                Particle::Water.get_color();
        } else if screen.buffer[x + positive_index + (y + 1) * screen.width]
            == Particle::Background.get_color()
        {
            screen.buffer[x + y * screen.width] = Particle::Background.get_color();
            screen.buffer[x + positive_index + (y + 1) * screen.width] =
                Particle::Water.get_color();
        }
    }
}

pub fn fluid_cascade(screen: &mut Screen, x: usize, y: usize) {
    match check_postion(screen, x) {
        Position::LeftSide => fluid_cascade_to_the_right(screen, x, y),
        Position::RightSide => fluid_cascade_to_the_left(screen, x, y),
        Position::Middle => {
            fluid_cascade_to_the_left(screen, x, y);
            fluid_cascade_to_the_right(screen, x, y);
        }
    }
}

fn fluid_cascade_to_the_left(screen: &mut Screen, x: usize, y: usize) {
    if screen.buffer[(x + 1) + y * screen.width] == Particle::Background.get_color() {
        screen.buffer[(x + 1) + y * screen.width] = screen.buffer[x + y * screen.width];
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}

fn fluid_cascade_to_the_right(screen: &mut Screen, x: usize, y: usize) {
    if screen.buffer[(x - 1) + y * screen.width] == Particle::Background.get_color() {
        screen.buffer[(x - 1) + y * screen.width] = screen.buffer[x + y * screen.width];
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}
