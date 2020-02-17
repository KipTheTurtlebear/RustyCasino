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


    // Game Loop: Continues until player chooses to exit the game
    while game == 'y' {
        //Deck of 52 cards created
        //Shuffle new deck every new game
        let mut deck: Deck = Deck::new_deck();
        deck.shuffle_deck();

        //Draw and print card the player sees
        let temp_card1 = deck.draw();
        println!("The card is a ");
        print_card(temp_card1);

        //Player chooses if it's higher or lower
        while choice != 1 && choice != 2 {
            println!("Do you think the next card will be higher or lower?\n1 = Higher, 2 = Lower\n");
            choice = read!();
        }

        //Draw 2nd card
        let temp_card2 = deck.draw();
        println!("The card is a ");
        print_card(temp_card2);

        //Compare 1st and 2nd card
        if get_value(temp_card2) > get_value(temp_card1) {
            result = 1; //Card was higher
        } else if get_value(temp_card2) < get_value(temp_card1) {
            result = 2; //Card was lower
        } else if get_value(temp_card2) == get_value(temp_card1) {
            result = 3; //Card was the same
        }


        if choice == result {println!("You win!");}
        else if result == 3 {println!("Card was same value, it's a draw!");}
        else {println!("You lose :("); }

        //Reset choice
        choice = 0;

        println!("Continue? y/n");

        game = read!();
    }
}
