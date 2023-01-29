use std::io::stdin;

use crate::{deck::Deck, pile::Pile, player::Player};

#[derive(Default, Clone)]
pub struct Game {
    deck: Deck,
    pile: Pile,
    players: Vec<Player>,
}
impl Game {
    pub fn new(players: usize) -> Self {
        let game = Game {
            players: vec![Player::default(); players],
            deck: Deck::new().shuffle(),
            ..Game::default()
        };

        // Deal cards
        game.initialize()
    }

    fn initialize(self) -> Self {
        let mut deck = self.deck.clone();
        Self {
            players: self
                .players
                .iter()
                .map(|uninitialized_p| {
                    let mut p = uninitialized_p.clone();
                    p.draw(&mut deck);
                    p.draw(&mut deck);
                    p.draw(&mut deck);

                    p
                })
                .collect(),
            deck,
            ..self
        }
    }

    pub(crate) fn play_turn(&self) -> &Self {
        /* loop { */
        println!("Your cards:");
        println!("{}", self.current_player().hand);
        println!("Play which card? ");

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let chosen_number = choice.parse::<u8>();
        /* break
        } */

        self
    }

    pub(crate) fn state(&self) -> State {
        match self.victor() {
            None => State::Playing,
            Some(player) => State::Completed(player),
        }
    }

    fn current_player(&self) -> &Player {
        self.players.first().unwrap()
    }

    fn victor(&self) -> Option<&Player> {
        self.players.iter().find(|p| p.hand.cards.is_empty())
    }
}

pub enum State<'a> {
    Playing,
    Completed(&'a Player),
}
