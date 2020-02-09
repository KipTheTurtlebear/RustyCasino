use rand::*;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;

#[derive(Debug, Default, Clone)]
pub struct Deck(Vec<i32>);

impl Deck{
    /// Instantiate a new deck of cards
    pub fn new_deck() -> Self {Deck((1..=52).collect())}

    /// Shuffle the deck
    pub fn shuffle(mut self) -> Self {
        let mut rng = thread_rng();
        let mut irs = Irs::default();

        irs.shuffle(&mut self.0, &mut rng);
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

    pub fn show_deck(&self) {
        let mut deck = &self.0.into_iter();

        for card in deck {
            if card <= 13 {
                println!("{} of Spades", card);
            }
            else if card > 13 && card <= 26 {
                println!("{} of Clubs", card - 13);
            }
            else if card > 26 && card <= 39 {
                println!("{} of Hearts", card - 26);
            }

            else {
                println!("{} of Diamonds", card - 39)
            }
        }
    }
}