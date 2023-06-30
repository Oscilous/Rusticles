use crate::Particle;
use crate::Screen;

pub fn gravity(screen: &mut Screen, x: usize, y: usize) {
    screen.buffer[x + (y + 1) * screen.width] = Particle::Sand.get_color();
    screen.buffer[x + y * screen.width] = Particle::Background.get_color();
}

pub fn middle_stacking(screen: &mut Screen, x: usize, y: usize) {
    if screen.buffer[(x + 1) + (y + 1) * screen.width] == Particle::Background.get_color() {
        screen.buffer[(x + 1) + y * screen.width] = Particle::Sand.get_color();
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    } else if screen.buffer[(x - 1) + (y + 1) * screen.width] == Particle::Background.get_color() {
        screen.buffer[(x - 1) + y * screen.width] = Particle::Sand.get_color();
        screen.buffer[x + y * screen.width] = Particle::Background.get_color();
    }
}
pub fn right_corner_stacking(screen: &mut Screen, x: usize, y: usize) {}
pub fn left_corner_stacking(screen: &mut Screen, x: usize, y: usize) {}
