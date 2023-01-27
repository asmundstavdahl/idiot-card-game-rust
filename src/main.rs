mod cards;
mod deck;
mod game;
mod pile;
mod player;

use game::{Game, State};

fn main() {
    let mut game = &Game::new(2);

    loop {
        match game.state() {
            State::Playing => game = game.play_turn(),
            State::Completed(victor) => {
                println!("Congratulations, {victor}!");
                break;
            }
        }
    }
}
