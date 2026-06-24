use game::snake::Snake;
use macroquad::prelude::*;

mod game;

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut snake = Snake::new();

    println!("Hello game!");
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
