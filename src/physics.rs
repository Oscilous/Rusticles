pub fn gravity(Screen: &mut Screen, x: usize, y: usize) {
    Screen.buffer[x + (y + 1) * Screen.WIDTH] = Particle::Sand.get_color();
    Screen.buffer[x + y * Screen.WIDTH] = Particle::Background.get_color();
}

pub fn middle_stacking(Screen: &mut Screen, x: usize, y: usize) {
    if Screen.buffer[(x + 1) + (y + 1) * Screen.WIDTH] == Particle::Background.get_color() {
        Screen.buffer[(x + 1) + y * Screen.WIDTH] = Particle::Sand.get_color();
        Screen.buffer[x + y * Screen.WIDTH] = Particle::Background.get_color();
    } else if Screen.buffer[(x - 1) + (y + 1) * Screen.WIDTH] == Particle::Background.get_color() {
        Screen.buffer[(x - 1) + y * Screen.WIDTH] = Particle::Sand.get_color();
        Screen.buffer[x + y * Screen.WIDTH] = Particle::Background.get_color();
    }
}
pub fn right_corner_stacking(Screen: &mut Screen, x: usize, y: usize) {}
pub fn left_corner_stacking(Screen: &mut Screen, x: usize, y: usize) {}
