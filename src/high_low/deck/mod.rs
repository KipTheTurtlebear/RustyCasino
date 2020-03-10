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

    ///draw card - gets rid of the card 'at the top' from the deck and returns it
    pub fn draw(&mut self) -> i32 {
        let a = self.0.pop();
        let card: i32 = a.unwrap();
        card
    }

    ///displays every card and their corresponding suite
    pub fn show_deck(&self) {
        let deck = &self.0;

        for card in deck {
            print_card(*card);
        }
    }
}

//had to move these functions out of the deck impl because they dont contain
//a 'self' so it doesn't count as a part of the deck struct implementation.
//still usable as long as the deck crate is imported.

    pub fn get_total (deck: &Vec<i32>) -> i32 {
        let mut value = 0;
        let mut aces = 0;
        let mut total = 0;
        for card in deck {
            value = get_bj_value(*card);
            if value == 11 {
                aces = aces + 1;
            }
            total = total + value;
        }
        while total > 21 && aces > 0 {
            total = total - 10;
            aces = aces - 1;
        }
        total
    }

    pub fn get_bj_value (card: i32) -> i32 {
        let mut value = card;
        if card == 1 || card == 14 || card == 27 || card == 40 {
            value = 11;
        }
        if card >= 2 && card <= 10 {
            value = card;
        } else if card >=11 && card <= 13 {
            value = 10;
        } else if card >= 15 && card <= 23 {
            value = card - 13;
        } else if card >= 24 && card <= 26 {
            value = 10;
        } else if card >= 28 && card <= 36 {
            value = card - 26;
        } else if card >= 37 && card <= 39 {
            value = 10;
        } else if card >= 41 && card <= 49 {
            value = card - 39;
        } else if card >= 50 && card <= 52 {
            value = 10;
        }
        value
    }
    pub fn get_value (card: i32) -> i32 {
        let mut value = card;
        if card >= 1 && card <= 13 {
            value = card;
        } else if card >= 14 && card <= 26 {
            value = card - 13;
        } else if card >= 27 && card <= 39 {
            value = card - 26;
        } else if card >= 40 && card <= 52 {
            value = card - 39;
        }
        value
    }


    ///takes an i32 value from 1-52 and determines what suite it is
    pub fn print_card(card: i32) {
        if card == 1 {
            println!("Ace of Spades");
        } else if card > 1 && card <= 10 {
            println!("{} of Spades", card);
        } else if card == 11 {
            println!("Jack of Spades");
        } else if card == 12 {
            println!("Queen of Spades");
        } else if card == 13 {
            println!("King of Spades");
        } else if card == 14 {
            println!("Ace of Clubs")
        } else if card > 14 && card <= 23 {
            println!("{} of Clubs", card - 13);
        } else if card == 24 {
            println!("Jack of Clubs");
        } else if card == 25 {
            println!("Queen of Clubs");
        } else if card == 26 {
            println!("King of Clubs");
        } else if card == 27 {
            println!("Ace of Hearts");
        } else if card > 27 && card <= 36 {
            println!("{} of Hearts", card - 26);
        } else if card == 37 {
            println!("Jack of Hearts");
        } else if card == 38 {
            println!("Queen of Hearts");
        } else if card == 39 {
            println!("King of Hearts");
        } else if card == 40 {
            println!("Ace of Diamonds");
        } else if card > 40 && card <= 49 {
            println!("{} of Diamonds", card - 39);
        } else if card == 50 {
            println!("Jack of Diamonds");
        } else if card == 51 {
            println!("Queen of Diamonds");
        } else {
            println!("King of Diamonds")
        }
    }



