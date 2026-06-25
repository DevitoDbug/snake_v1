use macroquad::prelude::*;

use crate::engine::{apple::Apple, collisions, helpers, snake::Snake};

enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    game_state: GameState,
    points: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Playing,
            points: 0,
        }
    }

    pub async fn start(&mut self) {
        let mut snake = Snake::new();
        let mut apple = Apple::new();
        let mut last_ran_time = 0.0;

        loop {
            clear_background(WHITE);

            Self::render_vert_lines(&self);

            match self.game_state {
                GameState::Playing => {
                    if get_time() - last_ran_time > 0.15 {
                        Self::render_game(&self, &mut snake, &mut apple, true);
                        last_ran_time = get_time();
                    } else {
                        Self::render_game(&self, &mut snake, &mut apple, false);
                    }
                }

                GameState::GameOver => Self::render_end_game(&self),
            }

            // Body collision check
            if snake.collided_with_body() {
                self.game_state = GameState::GameOver
            }

            // Wall collision check
            let head_pos = snake.get_head_pos();
            if head_pos.0 < 0.0 || head_pos.0 >= screen_width() {
                self.game_state = GameState::GameOver
            }
            if head_pos.1 < 0.0 || head_pos.1 >= screen_height() {
                self.game_state = GameState::GameOver
            }

            // Apple collision check
            let apple_pos = apple.get_apple_position();
            let is_collision = collisions::rect_vs_rect_collided(
                collisions::Rect {
                    x: head_pos.0,
                    y: head_pos.1,
                    height: helpers::get_block_size(),
                    width: helpers::get_block_size(),
                },
                collisions::Rect {
                    x: apple_pos.0,
                    y: apple_pos.1,
                    height: helpers::get_block_size(),
                    width: helpers::get_block_size(),
                },
            );

            if is_collision {
                apple.reset_position();
                snake.grow_snake();
                self.points += 1;
            }

            next_frame().await
        }
    }

    fn render_game(&self, snake: &mut Snake, apple: &Apple, move_snake: bool) {
        snake.render_snake();
        apple.render_apple();

        if move_snake {
            snake.move_snake();
        }
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
    }

    fn render_end_game(&self) {
        let x = screen_width() * 25.0 / 100.0;
        let y = screen_height() * 25.0 / 100.0;

        draw_text("Game over ", x, y, screen_width() * 15.0 / 100.0, RED);
    }

    fn render_vert_lines(&self) {
        let mut start = 0.0;
        while start < screen_width() {
            draw_line(start, 0.0, start, screen_height(), 1.0, GREEN);
            start += helpers::get_block_size();
        }

        let mut start = 0.0;
        while start < screen_height() {
            draw_line(0.0, start, screen_width(), start, 1.0, GREEN);
            start += helpers::get_block_size();
        }
    }
}
