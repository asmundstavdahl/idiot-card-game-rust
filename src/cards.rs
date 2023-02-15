use std::fmt::Display;

use enum_iterator::Sequence;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::game::SelectorString;

#[derive(Debug, Default, Clone)]
pub struct SetOfCards {
    pub cards: Vec<Card>,
}
impl SetOfCards {
    pub fn shuffle(&self) -> Self {
        let mut deck = self.clone();
        deck.cards.shuffle(&mut thread_rng());

        deck
    }

    pub fn add(&mut self, cards: &mut SetOfCards) {
        self.cards.append(&mut cards.cards)
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn extract_by_selector_string(
        &self,
        selector_string: SelectorString,
    ) -> (Option<Card>, Self) {
        (
            self.cards
                .iter()
                .filter(|c| c.selector_string().to_lowercase() == selector_string.to_lowercase())
                .last()
                .cloned(),
            Self {
                cards: self
                    .cards
                    .iter()
                    .filter(|c| {
                        c.selector_string().to_lowercase() != selector_string.to_lowercase()
                    })
                    .map(|c| c.to_owned())
                    .collect(),
            },
        )
    }

    pub fn present_cards(&self) -> String {
        let mut cards = self.cards.clone();
        cards.sort();
        cards.iter().map(|c| format!("{}\n", c.present())).collect()
    }
}
impl Display for SetOfCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|c| format!("{}\t{}", Card::selector_string(c), Card::to_string(c)))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct Card {
    pub value: CardValue,
    pub suit: Suit,
}
impl Card {
    pub fn selector_string(&self) -> String {
        format!(
            "{}{}",
            self.value.selector_string(),
            self.suit.selector_string(),
        )
    }

    pub fn present(&self) -> String {
        format!("{}\t{}", self.selector_string(), self.to_string())
    }
}
impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{:?} of {:?}", self.value, self.suit)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence, PartialOrd, Ord)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}
impl Suit {
    pub fn selector_string(self) -> &'static str {
        match self {
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
            Suit::Hearts => "H",
            Suit::Spades => "S",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Sequence, Ord)]
pub enum CardValue {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl CardValue {
    pub fn selector_string(self) -> &'static str {
        match self {
            CardValue::Ace => "A",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "10",
            CardValue::Jack => "J",
            CardValue::Queen => "Q",
            CardValue::King => "K",
        }
    }
}
