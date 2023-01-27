use enum_iterator::all;

use crate::cards::{Card, CardValue, SetOfCards, Suit};

pub type Deck = SetOfCards;
impl Deck {
    pub fn draw(&mut self, amount: i32) -> SetOfCards {
        let mut drawn = SetOfCards::default();

        for i in 0..amount {
            match self.pop() {
                None => {
                    println!("Can't draw another card. Intended to draw {amount} cards, {i} drawn successfully.");
                    break;
                }
                Some(card) => drawn.cards.push(card),
            }
        }

        drawn
    }

    pub fn new() -> Self {
        let mut deck = Deck { cards: vec![] };

        for suit in all::<Suit>().collect::<Vec<_>>() {
            all::<CardValue>()
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|value| {
                    deck.cards.push(Card {
                        suit: suit,
                        value: value,
                    })
                });
        }

        deck
    }
}
