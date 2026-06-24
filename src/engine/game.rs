use macroquad::prelude::*;

use crate::engine::snake::Snake;

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self) {
        let mut snake = Snake::new();
        loop {
            clear_background(WHITE);

            snake.render_snake();
            snake.move_snake();
            if is_key_pressed(KeyCode::Up) {
                snake.move_up();
            }
            if is_key_pressed(KeyCode::Down) {
                snake.move_down();
            }
            if is_key_pressed(KeyCode::Right) {
                snake.move_right();
            }
            if is_key_pressed(KeyCode::Left) {
                snake.move_left();
            }

            next_frame().await
        }
    }
}
