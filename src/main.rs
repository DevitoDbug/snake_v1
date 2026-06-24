use engine::game::Game;

mod engine;

#[macroquad::main("SnakeGame")]
async fn main() {
    let mut game = Game::new();
    game.start().await;
}
