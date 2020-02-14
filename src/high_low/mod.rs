//high-low module
mod deck;
mod player;
use crate::high_low::player::*;
use crate::high_low::deck::*;

//test to ensure this module linking stuff is working correctly
pub fn high_low(){

    //create player
    let mut player: Player = Player::new_player();

    println!("high-low game starting");

    //deck of 52 cards created
    let mut deck: Deck = Deck::new_deck();

    deck.shuffle_deck();

    let temp_card = deck.draw();
    println!("you drew a {}", temp_card);
    player.add_to_hand(temp_card); 

}
