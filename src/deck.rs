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
        Deck {
            cards: all::<Suit>()
                .flat_map(|suit| all::<CardValue>().map(move |value| Card { suit, value }))
                .collect(),
        }
    }
}
