//use rand::*;
//use shuffle::shuffler::Shuffler;
//use shuffle::irs::Irs;
use rand::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Deck(Vec<i32>);

impl Deck {
    /// Instantiate a new deck of cards
    pub fn new_deck() -> Self {
        Deck((1..=52).collect())
    }

    /// Shuffle the deck
    pub fn shuffle_deck(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        //let mut irs = Irs::default();

        self.0.shuffle(&mut rng);
        self
    }

    /// Count the number of cards in the deck
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this deck empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///displays every card and their corresponding suite
    pub fn show_deck(&self) {
        let deck = &self.0;

        for card in deck {
            if *card <= 13 {
                println!("{} of Spades", card);
            } else if *card > 13 && *card <= 26 {
                println!("{} of Clubs", card - 13);
            } else if *card > 26 && *card <= 39 {
                println!("{} of Hearts", card - 26);
            } else {
                println!("{} of Diamonds", card - 39)
            }
        }
    }
}
