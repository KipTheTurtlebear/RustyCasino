//high-low module
mod deck;
pub(crate) mod player;
use crate::high_low::deck::*;
use crate::high_low::player::*;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use text_io::read;


pub fn blackjack() {

    //create player
    let mut player: Player = Player::new_player();
    let path = Path::new("save.txt");
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut iter = reader.lines();
    let mut name = match iter.next() {
        Some(T) => T.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(T) => T.unwrap(),
        None => "100".to_string(),
    };

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());

    let mut choice = 0;
    let mut game: char = 'y';
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
        display_cards(&player.0);

        dealer.add_to_hand(deck.draw());
        println!("\nDealer's face-up card:");
        display_cards(&dealer.0);
        println!("\nYour cards are:");
        player.add_to_hand(deck.draw());
        display_cards(&player.0);

        dealer.add_to_hand(deck.draw());

        if get_total(&player.0) == 21 && get_total(&dealer.0) != 21 {
            println!("Blackjack! Congrats, you win!")
        } else if get_total(&player.0) != 21 && get_total(&dealer.0) == 21 {
            println!("Dealer got blackjack! You lose!")
        } else {
            println!("Want another card? y/n");
            hit = read!();

            while (get_total(&player.0) <= 21) && (hit == 'y') {
                player.add_to_hand(deck.draw());
                display_cards(&player.0);
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
                    println!("Dealer's cards:\n");
                    display_cards(&dealer.0);
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
pub fn high_low() {
    //create player
    let mut player: Player = Player::new_player();
    let path = Path::new("save.txt");
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut iter = reader.lines();

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
    let name = vec_lines[0].clone();

    let chips = vec_lines[1].clone();
};


    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());
    let mut choice = 0;
    let mut game: char = 'y';
    let mut result = 0;
    let mut bet:i32 = 0;
    let mut double = true;
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
            println!(
                "Do you think the next card will be higher or lower?\n1 = Higher, 2 = Lower\n"
            );
            choice = read!();

        let mut exceed = true;
        println!("You have: {} chips\n", player.1);
        while exceed {
            println!("How much would you like to bet?\n");
            let input:String = read!();
            let g = match input.parse::<i32>() {
                Err(e) => exceed = true,
                Ok(f) => bet = f,
            };
            if player.check_chips(bet) {exceed = false;}
        }
        //returns true if numeric
        /*
        let is_num = bet.parse::<i32>().is_ok();

        if is_num {
            let check = player.check_chips(bet);

        }
        while !is_num || !check {
            println!("You have: {} chips\n", player.1);
            println!("Please enter a number less than your chip count:\n");
            bet = read!();
            //returns true if numeric
            let is_num = bet;
            if is_num {
                let check = player.check_chips(bet);
            }



        }
        */
        //subtract bet
        player.lose_chips(bet);
        //Deck of 52 cards created
        //Shuffle new deck every new game
        if double {
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



            if choice == result {
                println!("You win!");
                bet = bet * 2;
                println!("Double or nothing? y/n\n");
                game = read!();
                if game == 'y' {
                    double = true;
                }
                else {
                    double = false;
                    println!("You've won {} chips!", bet);
                    player.add_chips(bet)
                }
            } else if result == 3 {
                println!("Card was same value, it's a draw!");
            }
            else {
                println!("You lose :(");
            }
        }
        //Reset choice
        double = true;
        choice = 0;

        println!("Continue? y/n");

        game = read!();
    }
    write_file(player.2, player.1);
}

//war!
pub fn war() {
    let bet_amount: i32 = 5;
    let mut game: char = 'y';

    println!("War game starting...\n");

    //initialize / set up the game

    let mut deck: Deck = Deck::new_deck();
    deck.shuffle_deck();

    //split deck between player and dealer
    let mut player_deck: Deck = Deck::empty_deck();
    let mut dealer_deck: Deck = Deck::empty_deck();

    //deal cards to each person until 'deck' is empty
    while !deck.is_empty() {
        player_deck.add_card(deck.draw());
        dealer_deck.add_card(deck.draw());
    }

    //TODO: Have Reginald tell player the game is gonna start, and how it works

    //after every 'round' the player can choose to either
    //continue
    //increase bet
    //decrease bet
    //but only in increments of 5 or 10 depending on how Reginald is feeling
    println!("What would you like the starting bet be? Default is 5 chips");

    //1 = dealer, 2 = player, 3 = tie
    let mut winner = 3;
    let mut choice = 0;

    //start game loop
    while game == 'y' {
        //draw
        let mut d_card = dealer_deck.draw();
        let mut p_card = player_deck.draw();

        //determine winner
        winner = war_winner(bet_amount, d_card, p_card);
        while winner == 3 {
            //if tie keep looping,
            //burn three cards
            dealer_deck.draw();
            dealer_deck.draw();
            dealer_deck.draw();
            player_deck.draw();
            player_deck.draw();
            player_deck.draw();

            d_card = dealer_deck.draw();
            p_card = dealer_deck.draw();

            winner = war_winner(bet_amount, d_card, p_card);
        }

        println!("winner returned {}", winner);

        println!("Would you like to go another round? y/n");
        game = read!();
        winner = 3;
    }
}

///Used to handle displaying the winner - returns a number representing who won / tie
pub fn war_winner(bet: i32, d_card: i32, p_card: i32) -> i32 {
    //0 means forfeit tie
    //1 means dealer wins
    //2 means player wins
    //3 means its war time
    let mut winner = 0;
    let mut choice = 1;
    println!("   Reginald's Card");
    display_single(d_card);

    if get_value(d_card) == get_value(p_card) {
        //its a tie!
        println!(" .. Oh?");
        println!("   Your Card");
        display_single(p_card);

        println!("\n\n\tIt's a tie! Well.. would you like to forfeit or start a war?\n1: War\n2: Forfeit");
        choice = read!();

        if choice == 2 {
            //forfeit tie - lose half
            println!("\n Ah.. That's too bad.");
            println!("\nThat means I get half");
            0
        } else {
            //war!
            println!("War! Now we burn three cards");
            3
        }
    } else if get_value(d_card) > get_value(p_card) {
        //dealer wins
        println!(" Dealer Wins!");
        println!("   Your Card");
        display_single(p_card);
        1
    } else {
        //player wins
        println!(" You Win!");
        println!("   Your Card");
        display_single(p_card);
        2
    }
}

///Red Dog Poker
pub fn red_dog_poker() {
    println!("Red Dog Poker Starting..");
    let mut deck: Deck = Deck::new_deck();

    let mut game: char = 'y';
    let mut bet_amount: i32 = 10;
    let mut button = 0;

    println!("Would you like to hear the rules? y/n?");
    game = read!();

    if game == 'y' {
        println!("You're allowed to bet 2 times per round.\nThe first time is when you're dealt 2 cards.");
        println!("Then depending on those two, you either double down or call.");
        println!("if you call, you get whatever payout for the first two cards.\nif you double down, you get dealt a 3rd card.");

        println!("Heres how the cards work");

        println!(
            "\n\t [ a ] [ b ] [ c ] say a, b, and c represent the first, second and third cards."
        );
        println!("\n\t You lose when a and b are not the same, and aren't consecutive, and when c is either higher or lower than either a or b");
        println!("Otherwise you want to look if the cards are\n1: The Same\n2: Consecutive\n3: Or if c is in between a and b");
        println!("If all three are the same, payout is 1:11\nIf a and b are the same, but c is different, it's a push
            \nIf a and b are consecutive, then you push and aren't dealt a 3rd card\nIf a and b aren't consecutive, and they aren't the same,
            then it's a spread. If c lands in between a and b, your payout depends on the size of that spread.");

        println!("\nAre you ready to play? [type anything and hit enter to continue]");
        button = read!();
    }

    game = 'y';
    print!("\n------ Let's Play Red Dog Poker! ------\n");

    deck.shuffle_deck();

    let mut player: Player = Player::new_player();

    while game == 'y' {
        println!("\nWhat would you like to bet? (Default 10)");

        //deal player two cards
        player.add_to_hand(deck.draw());
        player.add_to_hand(deck.draw());
        //display hand
        println!("\n\tYour Hand:");
        display_cards(&player.0);

        //ask if they'd like to double down, or call
        println!("Would you like to: \n1: double down\n2: call");
        button = read!();

        if button == 1 {
            player.add_to_hand(deck.draw());
            println!("\n\tYour Hand:");
            display_cards(&player.0);
        }
        //process payout
        //discard cards
        player.discard_hand();

        //^put in game loop
        println!("\nContinue? y/n");
        game = read!();
    }
}


