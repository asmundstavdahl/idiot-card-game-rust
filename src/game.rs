use std::{
    cmp,
    io::{stdin, stdout, Write},
};

use crate::{
    cards::{Card, CardValue, SetOfCards},
    deck::Deck,
    pile::Pile,
    player::Player,
};

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
        let mut pile = self.pile.clone();
        pile.add(&mut deck.draw(1));
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
            pile,
        }
    }

    pub(crate) fn play_turn(&self) -> Self {
        let mut current_player = self.players[0].clone();
        let other_players = &self.players[1..];

        let mut deck = self.deck.clone();
        let mut pile = self.pile.clone();

        loop {
            println!("Pile:\n{}", pile.present());
            println!("Your hand:\n{}", current_player.hand.present_cards());
            print!("Play which card?\n> ");
            let _ = stdout().flush();
            let mut buf = String::default();
            let _ = stdin().read_line(&mut buf);
            let buf = buf.trim().into();
            // can card be played?
            // take card from player's hand
            let (selected_card, other_cards) = current_player.hand.extract_by_selector_string(buf);
            let played_card = selected_card.unwrap();

            match self.player_can_play_card(&current_player, &played_card) {
                Err(reason) => {
                    println!("{}", reason);
                    continue;
                }
                Ok(()) => {
                    // put card onto pile
                    pile.add(&mut SetOfCards {
                        cards: vec![played_card],
                    });
                    current_player.hand = other_cards;
                    // fill player's hand
                    let missing_cards = cmp::max(0, 3 - current_player.hand.cards.len());
                    println!("{} draws {} card{}.", current_player.name, missing_cards, {
                        if missing_cards > 1 {
                            "s"
                        } else {
                            ""
                        }
                    });
                    current_player
                        .hand
                        .add(&mut deck.draw(missing_cards.try_into().unwrap()));

                    return Self {
                        players: other_players
                            .iter()
                            .cloned()
                            .chain([current_player].iter().cloned())
                            .collect(),
                        deck,
                        pile,
                    };
                }
            }
        }
    }

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

    fn player_can_play_card(&self, player: &Player, card: &Card) -> Result<(), &'static str> {
        let value_to_beat = match self.pile.cards.last() {
            Some(card) => card.value,
            None => CardValue::Ace,
        };

        if card.value >= value_to_beat {
            Ok(())
        } else {
            Err("Card is too weak")
        }
    }
}

pub enum State {
    Playing,
    Completed(Player),
}
