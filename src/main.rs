use rusty_engine::prelude::*;

struct GameState {
    health_left: i32,
}

fn main() {
    let mut game = Game::new();

    println!("Hello, world!");

    game.run(GameState { health_left: 42 });
}
