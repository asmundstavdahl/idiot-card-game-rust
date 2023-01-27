use crate::{deck::Deck, pile::Pile, player::Player};

#[derive(Default, Clone)]
pub struct Game {
    deck: Deck,
    pile: Pile,
    players: Vec<Player>,
}
impl Game {
    pub fn new(players: usize) -> Self {
        let game = Game {
            players: vec![Player::default(); players],
            ..Game::default()
        };

        // Deal cards
        game.initialize()
    }

    fn initialize(self) -> Self {
        self
    }

    pub(crate) fn over(&self) -> bool {
        todo!();
        false
    }

    pub(crate) fn play_turn(&self) -> &Self {
        println!("Your cards:");
        println!("{:?}", self.current_player().hand);

        self
    }

    pub(crate) fn state(&self) -> State {
        State::Playing
    }

    fn current_player(&self) -> &Player {
        self.players.first().unwrap()
    }
}

pub enum State<'a> {
    Playing,
    Completed(&'a str),
}
