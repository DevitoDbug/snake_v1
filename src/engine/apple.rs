use macroquad::{prelude::*, rand::gen_range};

use crate::engine::helpers;

pub struct Apple {
    x: f32,
    y: f32,
}

impl Apple {
    pub fn new() -> Self {
        Self {
            x: gen_range(0.0, screen_width()),
            y: gen_range(0.0, screen_height()),
        }
    }

    pub fn reset_position(&mut self) {
        self.x = gen_range(0.0, screen_width());
        self.y = gen_range(0.0, screen_height());
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
