mod cards;
mod deck;
mod game;
mod pile;
mod player;
mod term;

use game::{Game, State};

fn main() {
    let mut game = Game::new(2);

    print!("{}", term::clear());

    loop {
        match game.state() {
            State::Playing => game = game.play(),
            State::Completed(victor) => {
                println!("Congratulations, {}!", victor.name);
                break;
            }
        }
    }
}
