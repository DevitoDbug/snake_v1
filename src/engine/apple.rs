use macroquad::prelude::*;

use crate::engine::helpers;

struct Apple {
    x: f32,
    y: f32,
}

impl Apple {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn _reset_position(&mut self) {
        let max_width = screen_width();
        let max_height = screen_height();
        let max_width_i32 = max_width as i32;
        let max_height_i32 = max_height as i32;

        let x = rand::gen_range(1, max_width_i32);
        let y = rand::gen_range(1, max_height_i32);

        self.x = x as f32;
        self.y = y as f32;
    }

    fn _render_apple(self) {
        draw_rectangle(
            self.x,
            self.y,
            helpers::get_block_size(),
            helpers::get_block_size(),
            PURPLE,
        );
    }
}
