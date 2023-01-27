use super::cards::{Card, SetOfCards};

pub type Pile = SetOfCards;
impl Pile {
    fn play(&mut self, card: Card) -> Result<(), ()> {
        if self.can_play(&card) {
            self.add(&mut SetOfCards { cards: vec![card] });
            Ok(())
        } else {
            Err(())
        }
    }

    fn can_play(&self, candidate_card: &Card) -> bool {
        match self.cards.last() {
            None => true,
            Some(pile_card) => candidate_card.value >= pile_card.value,
        }
    }
}
