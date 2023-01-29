

use rand::random;

use super::{cards::SetOfCards, deck::Deck};

#[derive(Clone, Debug)]
pub struct Player {
    pub hand: SetOfCards,
    pub name: String,
}
impl Player {
    pub fn draw(&mut self, deck: &mut Deck) {
        self.hand.add(&mut deck.draw(1))
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hand: SetOfCards::default(),
            name: format!("Player {}", 1 + random::<u8>() % 9)
        }
    }
}
