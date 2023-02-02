use rand::random;

use super::{cards::SetOfCards, deck::Deck};

#[derive(Clone, Debug)]
pub struct Player {
    pub hand: SetOfCards,
    pub name: String,
}
impl Player {
    pub fn new(name: String) -> Self {
        Self {
            hand: SetOfCards::default(),
            name,
        }
    }

    pub fn draw(&self, deck: &mut Deck) -> Self {
        let mut hand = self.hand.clone();
        hand.add(&mut deck.draw(1));

        Self {
            hand,
            ..self.clone()
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hand: SetOfCards::default(),
            name: format!("Player {}", 1 + random::<u8>() % 9),
        }
    }
}
