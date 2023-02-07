use super::cards::SetOfCards;

pub type Pile = SetOfCards;
impl Pile {
    pub fn present(&self) -> String {
        let top_card = match self.cards.last() {
            Some(card) => card.present(),
            None => "Empty".into(),
        };

        format!("{} ({} card(s))", top_card, self.cards.len())
    }
}
