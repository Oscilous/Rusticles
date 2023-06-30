pub fn gravity(buffer: &mut Vec<u32>, x: usize, y: usize, SAND_COLOR: usize) {
    buffer[x + (y + 1) * WIDTH] = SAND_COLOR;
    buffer[x + y * WIDTH] = BACKGROUND_COLOR;
}

pub fn middle_stacking(buffer: &mut Vec<u32>, x: usize, y: usize) {
    if buffer[(x + 1) + (y + 1) * WIDTH] == BACKGROUND_COLOR {
        buffer[(x + 1) + y * WIDTH] = SAND_COLOR;
        buffer[x + y * WIDTH] = BACKGROUND_COLOR;
    } else if buffer[(x - 1) + (y + 1) * WIDTH] == BACKGROUND_COLOR {
        buffer[(x - 1) + y * WIDTH] = SAND_COLOR;
        buffer[x + y * WIDTH] = BACKGROUND_COLOR;
    }
}
pub fn right_corner_stacking(buffer: &mut Vec<u32>, x: usize, y: usize) {}
pub fn left_corner_stacking(buffer: &mut Vec<u32>, x: usize, y: usize) {}
