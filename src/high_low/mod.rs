//high-low module
mod deck;
mod player;
use crate::high_low::player::*;
use crate::high_low::deck::*;
use text_io::read;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::error::Error;

pub fn blackjack(){

    let mut choice = 0;
    let mut game :char = 'y';
    let mut result = 0;
    println!("Blackjack game starting...\n");


    // Game Loop: Continues until player chooses to exit the game
    while game == 'y' {
        let mut player = Player::new_player();
        let mut dealer = Player::new_player();
        let chips = 100;

        player.set_name("Player".to_string());
        player.add_chips(chips);
        //Deck of 52 cards created
        //Shuffle new deck every new game
        let mut deck: Deck = Deck::new_deck();
        let mut hit: char = 'y';
        deck.shuffle_deck();

        println!("Dealing...");
        player.add_to_hand(deck.draw());
        println!("\nYour first card:");
        player.show_hand();

        dealer.add_to_hand(deck.draw());
        println!("\nDealer's face-up card:");
        dealer.show_hand();
        println!("\nYour cards are:");
        player.add_to_hand(deck.draw());
        player.show_hand();

        dealer.add_to_hand(deck.draw());

        if get_total(&player.0) == 21 && get_total(&dealer.0) != 21 {
            println!("Blackjack! Congrats, you win!")
        }
        else if get_total(&player.0) != 21 && get_total(&dealer.0) == 21 {
            println!("Dealer got blackjack! You lose!")
        }

        else {
            println!("Want another card? y/n");
            hit = read!();


            while (get_total(&player.0) <= 21) && (hit == 'y') {
                player.add_to_hand(deck.draw());
                player.show_hand();
                println!("Want another card? y/n");
                hit = read!();
            }

            if get_total(&player.0) > 21 {
                println!("BUSTED! Sucks to suck yo, better luck next time.")
            } else {
                println!("Your final count is: {}", get_total(&player.0));
                println!("Dealer is drawing:");
                while get_total(&dealer.0) <= get_total(&player.0) {
                    print!("\nDealer's cards:\n");
                    dealer.add_to_hand(deck.draw());
                    dealer.show_hand();
                }
                println!("Dealer's final count is: {}", get_total(&dealer.0));
                if get_total(&dealer.0) > 21 {
                    println!("\nDealer busted, you win!\n");
                } else {
                    if get_total(&player.0) > get_total(&dealer.0) {
                        println!("\nYou win!\n");
                    } else if get_total(&player.0) < get_total(&dealer.0) {
                        println!("\nYou lose!\n");
                    } else {
                        println!("\nIt's a tie!\n")
                    }
                }
            }
        }
        println!("\nDo you want to play again?\n");
        game = read!();
    }
}

//test to ensure this module linking stuff is working correctly
pub fn high_low(){

    //create player
    let mut player: Player = Player::new_player();
    let path = Path::new("save.txt");
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
   /* let mut iter = reader.lines();
    let mut name = match iter.next() {
        Some(T) => T.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(T) => T.unwrap(),
        None => "100".to_string(),
    };*/
    let mut vec_lines = vec![];
    for line in reader.lines() {
        vec_lines.push(line.unwrap());
    }
    let name =  vec_lines[0].clone();

    let chips = vec_lines[1].clone();

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());
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
        display_single(temp_card1);

        //Player chooses if it's higher or lower
        while choice != 1 && choice != 2 {
            println!("Do you think the next card will be higher or lower?\n1 = Higher, 2 = Lower\n");
            choice = read!();
        }

        //Draw 2nd card
        let temp_card2 = deck.draw();
        println!("The card is a ");
        print_card(temp_card2);
        display_single(temp_card2);

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

//war!
pub fn war(){

    //let bet_amount:i32 = 0; ?
    let mut game: char = 'y';

    println!("War game starting...\n");

    //initialize / set up the game

    let mut deck: Deck = Deck::new_deck();
    deck.shuffle_deck();

    //split deck between player and dealer
    let mut player_deck: Deck = Deck::empty_deck();
    let mut dealer_deck: Deck = Deck::empty_deck();

    //deal cards to each person until 'deck' is empty
    while !deck.is_empty(){
        player_deck.add_card(deck.draw());
        dealer_deck.add_card(deck.draw());
    }




    //start game loop

}
