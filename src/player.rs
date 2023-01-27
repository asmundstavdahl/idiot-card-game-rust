use super::{cards::SetOfCards, deck::Deck};

#[derive(Default, Clone)]
pub struct Player {
    pub hand: SetOfCards,
}
impl Player {
    pub fn draw(&mut self, deck: &mut Deck) {
        self.hand.add(&mut deck.draw(1))
    }
}
