use macroquad::{color::RED, shapes::draw_rectangle};

use crate::engine::{collisions, helpers};

pub struct Snake {
    body: Vec<(f32, f32)>,
    grow: bool,

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
            grow: false,

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

        if self.grow {
            let new_head_x = self.body[head_index].0 + self.dx;
            let new_head_y = self.body[head_index].1 + self.dy;
            self.body.push((new_head_x, new_head_y));
        }
    }

    pub fn grow_snake(&mut self) {
        self.grow = true;
    }

    pub fn collided_with_body(&self) -> bool {
        let head = self.get_head_pos();
        let block_size = helpers::get_block_size();

        for (i, body_part) in self.body.iter().enumerate() {
            if i == self.body.len() - 1 {
                continue;
            }
            let body_head_collided = collisions::rect_vs_rect_collided(
                collisions::Rect {
                    x: head.0,
                    y: head.1,
                    width: block_size,
                    height: block_size,
                },
                collisions::Rect {
                    x: body_part.0,
                    y: body_part.1,
                    width: block_size,
                    height: block_size,
                },
            );

            if body_head_collided {
                return true;
            }
        }

        false
    }

    pub fn move_up(&mut self) {
        if self.dy != 0.0 {
            return;
        }

        self.dx = 0.0;
        self.dy = -helpers::get_block_size();
    }

    pub fn move_down(&mut self) {
        if self.dy != 0.0 {
            return;
        }
        self.dx = 0.0;
        self.dy = helpers::get_block_size();
    }

    pub fn move_right(&mut self) {
        if self.dx != 0.0 {
            return;
        }
        self.dx = helpers::get_block_size();
        self.dy = 0.0;
    }

    pub fn move_left(&mut self) {
        if self.dx != 0.0 {
            return;
        }
        self.dx = -helpers::get_block_size();
        self.dy = 0.0;
    }
}
