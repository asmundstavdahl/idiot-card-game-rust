use std::io::stdin;

use crate::{deck::Deck, pile::Pile, player::Player};

#[derive(Default, Clone)]
pub struct Game {
    deck: Deck,
    pile: Pile,
    players: Vec<Player>,
}
impl Game {
    pub fn new(number_of_players: usize) -> Self {
        let mut players: Vec<Player> = Vec::new();
        players.reserve(number_of_players);
        for i in 1..number_of_players {
            players.push(Player::new(format!("Player {}", i)))
        }

        let game = Game {
            players,
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

    pub(crate) fn play_turn(self) -> Self {
        let current_player = self.clone().current_player();
        match current_player {
            None => self.play_turn(),
            Some(player) => self.play_turn_for_player(player.clone()),
        }
    }

    fn play_turn_for_player(self, player: Player) -> Self {
        /* loop { */
        println!("Your cards:");
        println!("{}", player.hand);
        println!("Play which card? ");

        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let chosen_number = choice.parse::<u8>();
        /* break
        } */

        let mut players_rotated_with_next_player_first = self.players.clone();
        players_rotated_with_next_player_first.rotate_left(1);

        Self {
            players: players_rotated_with_next_player_first,
            ..self.clone()
        }
    }

    pub(crate) fn state(&self) -> State {
        match self.victor() {
            None => State::Playing,
            Some(player) => State::Completed(player),
        }
    }

    fn current_player(self) -> Option<Player> {
        self.players.first().cloned()
    }

    fn victor(&self) -> Option<Player> {
        self.players
            .iter()
            .find(|p| p.hand.cards.is_empty())
            .cloned()
    }
}

pub enum State {
    Playing,
    Completed(Player),
}
