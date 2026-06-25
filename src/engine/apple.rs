use macroquad::prelude::*;

use crate::engine::helpers::{self, get_block_size};

pub struct Apple {
    x: f32,
    y: f32,
}

impl Apple {
    pub fn new() -> Self {
        let mut apple = Self { x: 0.0, y: 0.0 };
        apple.reset_position();

        return apple;
    }

    pub fn reset_position(&mut self) {
        let max_x_blocks = (screen_width() / get_block_size()) as i32;
        let random_x = rand::gen_range(0, max_x_blocks);
        let max_y_blocks = (screen_height() / get_block_size()) as i32;
        let random_y = rand::gen_range(0, max_y_blocks);

        self.x = get_block_size() * random_x as f32;
        self.y = get_block_size() * random_y as f32;
    }

    pub fn get_apple_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn render_apple(&self) {
        draw_rectangle(
            self.x,
            self.y,
            helpers::get_block_size(),
            helpers::get_block_size(),
            PURPLE,
        );
    }
}
