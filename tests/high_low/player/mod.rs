//player module
//use crate::high_low::deck;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Player(pub Vec<i32>, pub i32, pub String); //Hand of cards, chips, name

impl Player {
    ///Create the player
    pub fn new_player() -> Self {
        //don't really know if we need this
        // Player((0..0).collect())
        Player(Vec::new(), 0, String::new())
    }

    pub fn set_name(&mut self, name: String) {
        self.2 = name;
    }

    ///Add card to player's hand
    pub fn add_to_hand(&mut self, card: i32) {
        //push to hand
        self.0.push(card);
    }

    /*
     //unused functions
    pub fn show_hand(&self) {
        let hand = &self.0;

        for card in hand {
            deck::print_card(*card);
        }
    }
    */

    /*
    ///Remove card from player's hand
    pub fn remove_card(&mut self, to_remove: i32) {
        let mut count = 0;

        for card in &self.0 {
            if to_remove == *card {
                //found first instance of card we want to remove
                &self.0.remove(count);
                break;
            }

            //haven't found, so up the counter
            count += 1;
        }
    }
    */

    ///Discard hand
    pub fn discard_hand(&mut self) {
        while !self.0.is_empty() {
            self.0.pop();
        }
    }

    pub fn add_chips(&mut self, chips: i32) {
        match self.1.checked_add(chips) {
            Some(v) => {
                self.1 = v;
            }
            None => {
                println!("You're OVERFLOWING with chips");
            }
        }
    }

    ///removes chips upon loss
    pub fn lose_chips(&mut self, lose_amount: i32) {
        //do we want negative chips? let player buy in ?

        self.1 -= lose_amount;
    }

    ///Checks if player has enough chips to bet

    pub fn check_chips(&self, to_bet: i32) -> bool {
        //use this function where betting is handled.

        //if this returns false, user does not have enough chips to bet
        //so they need to change bet amount

        /*
        if to_bet > self.1 {
            false
        } else {
            true
        }
        */
        to_bet <= self.1
    }
}

pub fn write_file(name: String, chips: i32) {
    let mut textfile: String = name.clone();
    textfile.push_str(".txt");

    // Create file and save path
    let path = Path::new("save.txt");
    let display = path.display();

    // Open the file in write-mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(name.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("\n\n\t\"Ah, {:?}, what a wonderful name.\"\n", name),
    }
    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => file,
    };

    write!(file, "\n{}", chips).expect("Couldn't write to file");
}
