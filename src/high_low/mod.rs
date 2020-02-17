//high-low module
mod deck;
mod player;
use crate::high_low::player::*;
use crate::high_low::deck::*;
use text_io::read;

//test to ensure this module linking stuff is working correctly
pub fn high_low(){

    //create player
    let mut player: Player = Player::new_player();
    let mut choice = 0;
    let mut game :char = 'y';
    let mut result = 0;
    println!("high-low game starting");

    //deck of 52 cards created
    let mut deck: Deck = Deck::new_deck();
    while game == 'y' {
        deck.shuffle_deck();

        let temp_card1 = deck.draw();
        println!("The card is a ");
        print_card(temp_card1);

        while choice != 1 && choice != 2 {
            println!("Do you think the next card will be higher or lower?\n1 = Higher, 2 = Lower\n");
            choice = read!();
        }

        let temp_card2 = deck.draw();
        println!("The card is a ");
        print_card(temp_card2);

        if get_value(temp_card2) > get_value(temp_card1) {
            result = 1; //Card was higher
        }

        if choice == result {
            println!("You win!");
        }
        else { println!("You lose :("); }

        println!("Continue? y/n");
        game = read!();
    }
}
