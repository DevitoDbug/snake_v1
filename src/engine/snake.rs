use macroquad::{color::RED, shapes::draw_rectangle};

use crate::engine::helpers;

pub struct Snake {
    body: Vec<(f32, f32)>,

    dx: f32,
    dy: f32,
}

impl Snake {
    pub fn new() -> Self {
        let block_size = helpers::get_block_size();
        Self {
            body: vec![
                (0.0, block_size),
                (block_size * 1.0, block_size),
                (block_size * 2.0, block_size),
            ],

            dx: block_size,
            dy: 0.0,
        }
    }

    pub fn render_snake(&self) {
        let block_size = helpers::get_block_size();

        for block in &self.body {
            draw_rectangle(block.0, block.1, block_size, block_size, RED);
        }
    }

    pub fn get_head_pos(&self) -> (f32, f32) {
        let head_index = self.body.len() - 1;

        (self.body[head_index].0, self.body[head_index].1)
    }

    pub fn move_snake(&mut self) {
        let head_index = self.body.len() - 1;
        // Move the rest of the body first
        for i in 0..self.body.len() {
            if i < self.body.len() - 1 {
                self.body[i] = self.body[i + 1];
            }
        }

        // Move the head
        self.body[head_index].0 += self.dx;
        self.body[head_index].1 += self.dy;
    }

    pub fn move_up(&mut self) {
        self.dx = 0.0;
        self.dy = -helpers::get_block_size();
    }

    pub fn move_down(&mut self) {
        self.dx = 0.0;
        self.dy = helpers::get_block_size();
    }

    pub fn move_right(&mut self) {
        self.dx = helpers::get_block_size();
        self.dy = 0.0;
    }

    pub fn move_left(&mut self) {
        self.dx = -helpers::get_block_size();
        self.dy = 0.0;
    }
}
