//player module
use crate::high_low::deck;

#[derive(Debug, Default, Clone)]
pub struct Player(pub Vec<i32>, i32, String); //Hand of cards, chips, name

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

    pub fn show_hand(&self) {
        let hand = &self.0;

        for card in hand {
            deck::print_card(*card);
        }
    }

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
    pub fn lose_chips(mut self, lose_amount: i32) -> Self {
        //do we want negative chips? let player buy in ?

        self.1 = self.1 - lose_amount;
        self
    }

    ///Checks if player has enough chips to bet
    pub fn check_chips(self, to_bet: i32) -> bool {
        //use this function where betting is handled.
        //if this returns false, user does not have enough chips to bet
        //so they need to change bet amount
        if to_bet > self.1 {
            false
        } else {
            true
        }
    }
}
