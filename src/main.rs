mod asteroid;
mod asteroid_manager;
mod bullet;
mod game;
mod global;
mod ship;
use game::*;

fn main() {
    let mut game: Game = Game::new("BOXING_TYPING_GAME");
    game.main_loop();
}
