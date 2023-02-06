use std::{cmp, io::stdin};

use crate::{cards::SetOfCards, deck::Deck, pile::Pile, player::Player};

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
                    uninitialized_p
                        .clone()
                        .draw(&mut deck)
                        .draw(&mut deck)
                        .draw(&mut deck)
                })
                .collect(),
            deck,
            ..self
        }
    }

    pub(crate) fn play_turn(&self) -> Self {
        let mut current_player = self.players[0].clone();
        let other_players = &self.players[1..];

        let mut deck = self.deck.clone();
        let mut pile = self.pile.clone();

        println!("Pile:\n{}", pile.present_cards());
        println!("Your hand:\n{}", current_player.hand.present_cards());
        print!("Play which card?\n> ");
        let mut buf = String::default();
        let _ = stdin().read_line(&mut buf);
        // can card be played?
        // take card from player's hand
        let (selected_card, other_cards) = current_player.hand.extract_by_selector_string(buf);
        let played_card = selected_card.unwrap();
        // put card onto pile
        pile.add(&mut SetOfCards {
            cards: vec![played_card],
        });
        // fill player's hand
        let missing_cards = cmp::max(0, 3 - current_player.hand.cards.len());
        current_player
            .hand
            .add(&mut deck.draw(missing_cards.try_into().unwrap()));

        let new_current_player = Player {
            hand: other_cards,
            ..current_player
        };

        Self {
            players: other_players
                .iter()
                .cloned()
                .chain([new_current_player].iter().cloned())
                .collect(),
            deck,
            pile,
        }
    }
    /*
       fn play_turn_for_player(self, player: Player) -> Self {
           /* loop { */
           println!("Your cards:");
           println!("{}", player.present_cards());
           println!("Play which card? ");

           let mut choice = String::new();
           stdin().read_line(&mut choice).unwrap();

           let (extracted_card, other_cards) = player.hand.extract_by_selector_string(choice);

           if let Some(card) = extracted_card {
               return Self {};
           }
           /* break
           } */

           let mut players_rotated_with_next_player_first = self.players.clone();
           players_rotated_with_next_player_first.rotate_left(1);

           Self {
               players: players_rotated_with_next_player_first,
               ..self.clone()
           }
       }
    */
    pub(crate) fn state(&self) -> State {
        match self.victor() {
            None => State::Playing,
            Some(player) => State::Completed(player),
        }
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
