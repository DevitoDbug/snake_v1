use macroquad::window::screen_width;

pub fn get_block_size() -> f32 {
    screen_width() * 1.5 / 100.0
}
