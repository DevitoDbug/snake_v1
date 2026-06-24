use engine::game::Game;

mod engine;

#[macroquad::main("SnakeGame")]
async fn main() {
    let game = Game::new();
    game.start().await;
}
