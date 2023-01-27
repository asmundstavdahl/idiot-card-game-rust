use enum_iterator::Sequence;
use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Default, Clone)]
pub struct SetOfCards {
    pub cards: Vec<Card>,
}
impl SetOfCards {
    pub fn shuffle(&mut self) -> &mut Self {
        self.cards.shuffle(&mut thread_rng());
        self
    }

    pub fn add(&mut self, cards: &mut SetOfCards) {
        self.cards.append(&mut cards.cards)
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Sequence)]
pub enum CardValue {
    Ace = 1,
    One,
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
