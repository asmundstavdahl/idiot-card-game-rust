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
        for i in 1..=number_of_players {
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

    pub(crate) fn play(&self) -> Self {
        let mut current_player = self.players[0].clone();
        let other_players = &self.players[1..];

        let mut deck = self.deck.clone();
        let mut pile = self.pile.clone();

        print!("{}", term_clear());

        loop {
            let play_instruction = self.ask_player_for_play_instruction(&current_player);
            match play_instruction {
                PlayInstruction::Pass => {
                    current_player.hand.add(&mut pile);

                    break;
                }
                PlayInstruction::Selector(selector_string) => {
                    // can card be played?
                    // take card from player's hand
                    let (selected_card, other_cards) = current_player
                        .hand
                        .extract_by_selector_string(selector_string);
                    match selected_card {
                        None => continue,
                        Some(card) => {
                            match self.card_can_be_played(&card) {
                                Err(reason) => {
                                    term_error(&(reason + "\n"));
                                    continue;
                                }
                                Ok(()) => {
                                    // put card onto pile
                                    pile.add(&mut SetOfCards { cards: vec![card] });
                                    current_player.hand = other_cards;
                                    // fill player's hand
                                    let missing_cards =
                                        3 - cmp::min(3, current_player.hand.cards.len());
                                    println!(
                                        "{} draws {} card{}.",
                                        current_player.name,
                                        missing_cards,
                                        {
                                            if missing_cards > 1 {
                                                "s"
                                            } else {
                                                ""
                                            }
                                        }
                                    );
                                    current_player
                                        .hand
                                        .add(&mut deck.draw(missing_cards.try_into().unwrap()));

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        Self {
            players: [other_players, &[current_player]].concat(),
            deck,
            pile,
        }
    }

    fn ask_player_for_play_instruction(&self, player: &Player) -> PlayInstruction {
        println!("It's your turn, {}.", term_bold(&player.name));
        println!("Pile:\n{}", self.pile.present());
        println!("Your hand:\n{}", player.hand.present_cards());
        print!("Play which card? (PASS to draw pile)\n> ");
        let _ = stdout().flush();
        let mut buf = String::default();
        let _ = stdin().read_line(&mut buf);
        print!("{}", term_clear());
        let buf = buf.trim().into();
        if buf == "pass" {
            PlayInstruction::Pass
        } else {
            PlayInstruction::Selector(buf)
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

    fn card_can_be_played(&self, card: &Card) -> Result<(), String> {
        let value_to_beat = match self.pile.cards.last() {
            Some(card) => card.value,
            None => CardValue::Ace,
        };

        if card.value >= value_to_beat {
            Ok(())
        } else {
            Err(format!("Card {} is too weak", card.selector_string()))
        }
    }
}

fn term_clear() -> String {
    "\x1B[2J\x1B[f".to_string()
}

fn term_bold(msg: &String) -> String {
    format!("\x1B[1m{}\x1B[0m", msg)
}

fn term_invert(msg: &String) -> String {
    format!("\x1B[7m{}\x1B[0m", msg)
}

fn term_error(msg: &String) -> String {
    term_invert(&term_bold(msg))
}

pub enum State {
    Playing,
    Completed(Player),
}

enum PlayInstruction {
    Selector(SelectorString),
    Pass,
}
impl From<String> for PlayInstruction {
    fn from(value: String) -> Self {
        if value.to_lowercase() == "pass" {
            PlayInstruction::Pass
        } else {
            value.into()
        }
    }
}

pub(crate) type SelectorString = String;
