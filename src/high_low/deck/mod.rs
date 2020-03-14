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

    /// Creates an empty deck - used to create multiple decks
    pub fn empty_deck() -> Self {
        Deck(Vec::new())
    }

    /// Adds a card to the deck - used to create multiple decks
    pub fn add_card(&mut self, card: i32) -> &Self {
        self.0.push(card);
        self
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
        let mut a = self.0.pop();

        if a == None {
            println!("\n Let me just shuffle these cards real quick");
            self.0 = (1..=52).collect();
            self.shuffle_deck();
            a = self.0.pop();
        }
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

pub fn get_total(deck: &[i32]) -> i32 {
    //let mut value = 0;
    let mut aces = 0;
    let mut total = 0;
    for card in deck {
        let value = get_bj_value(*card);
        if value == 11 {
            aces += 1;
        }
        total += value;
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

///Returns the card's number value - Faces = 10
pub fn get_bj_value(card: i32) -> i32 {
    //let value = card;
    if card == 1 || card == 14 || card == 27 || card == 40 {
        //value = 11;
        11
    } else if card >= 2 && card <= 10 {
        //value = card;
        card
    } else if card >= 11 && card <= 13 {
        //value = 10;
        10
    } else if card >= 15 && card <= 23 {
        //value = card - 13;
        card - 13
    } else if card >= 24 && card <= 26 {
        //value = 10;
        10
    } else if card >= 28 && card <= 36 {
        //value = card - 26;
        card - 26
    } else if card >= 37 && card <= 39 {
        //value = 10;
        10
    } else if card >= 41 && card <= 49 {
        //value = card - 39;
        card - 39
    /*
    } else if card >= 50 && card <= 52 {
        //value = 10;
        10
    }
    */
    } else {
        10
    }
    //value
}

///Returns the card's number value
pub fn get_value(card: i32) -> i32 {
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

/// Returns a tuple containing the card's number value, name of suite, and unicode of suite
pub fn card_info(card: i32) -> (i32, String, char) {
    //get number value of card
    let mut info: (i32, String, char) = (0, "empty".to_string(), '\u{26A0}');
    info.0 = get_value(card);

    //get name of suite
    info.1 = get_suite(card);

    //get unicode of suite
    if info.1 == "Spades" {
        info.2 = '\u{2660}';
    } else if info.1 == "Clubs" {
        info.2 = '\u{2663}';
    } else if info.1 == "Hearts" {
        info.2 = '\u{2665}';
    } else {
        info.2 = '\u{2666}';
    }

    info
}

///Returns the literal string name of the suite given by the card
pub fn get_suite(card: i32) -> String {
    if card >= 1 && card <= 13 {
        "Spades".to_string()
    } else if card >= 14 && card <= 26 {
        "Clubs".to_string()
    } else if card >= 27 && card <= 39 {
        "Hearts".to_string()
    } else {
        "Diamonds".to_string()
    }
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

pub fn print_value(card: i32) {
    let num = get_value(card);

    if num >= 2 && num <= 10 {
        print!("{}", num);
    } else if num == 1 {
        print!("A");
    } else if num == 11 {
        print!("J");
    } else if num == 12 {
        print!("Q");
    } else if num == 13 {
        print!("K");
    }
}

///displays a single card
pub fn display_single(card: i32) {
    print!("|");
    if card == 10 || card == 23 || card == 36 || card == 49 {
        print_value(card);
        println!("  |");

        println!("|  {} |", card_info(card).2);

        print!("|  ");
    } else {
        print_value(card);
        println!("  |");

        println!("| {} |", card_info(card).2);

        print!("|  ");
    }
    print_value(card);
    println!("|");
}

///displays the cards in the users hand with a 'picture'
pub fn display_cards(cards: &[i32]) {
    //display top layer
    for card in cards {
        print!("|");
        print_value(*card);
        print!("  |  ");
    }

    println!();

    //display middle layer
    for card in cards {
        if *card == 10 || *card == 23 || *card == 36 || *card == 49 {
            print!("|  {} |  ", card_info(*card).2);
        } else {
            print!("| {} |  ", card_info(*card).2);
        }
    }
    println!();

    //display bottom layer
    for card in cards {
        print!("|  ");
        print_value(*card);
        print!("|  ");
    }
}

