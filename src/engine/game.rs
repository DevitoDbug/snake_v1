use macroquad::prelude::*;

use crate::engine::snake::Snake;

enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Playing,
        }
    }

    pub async fn start(&mut self) {
        let mut snake = Snake::new();
        loop {
            clear_background(WHITE);

            match self.game_state {
                GameState::Playing => Self::render_game(&self, &mut snake),
                GameState::GameOver => Self::render_end_game(&self),
            }

            // TODO:: Do better game end
            let head = snake.get_head_pos();
            if head.0 < 0.0 || head.0 > screen_width() {
                self.game_state = GameState::GameOver
            }
            if head.1 < 0.0 || head.1 > screen_height() {
                self.game_state = GameState::GameOver
            }

            next_frame().await
        }
    }

    fn render_game(&self, snake: &mut Snake) {
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
    }

    fn render_end_game(&self) {
        let x = screen_width() * 25.0 / 100.0;
        let y = screen_height() * 25.0 / 100.0;

        draw_text("Game over ", x, y, screen_width() * 15.0 / 100.0, RED);
    }
}
