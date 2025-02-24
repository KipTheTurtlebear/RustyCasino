//high-low module
mod deck;
pub(crate) mod player;
use crate::high_low::deck::*;
use crate::high_low::player::*;
use std::cmp;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use text_io::read;

pub fn get_bet() -> i32 {
    let mut bet = 0;
    println!("How much would you like to bet?\n");
    let input: String = read!();
    match input.parse::<i32>() {
        Err(_e) => println!("\n\nEnter a number less than or equal to your chip total\n"),
        Ok(f) => bet = f,
    };
    bet
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
    let name = match iter.next() {
        Some(t) => t.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(t) => t.unwrap(),
        None => "100".to_string(),
    };

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());
    let mut choice = 0;
    let mut result: i32;
    let mut game: char = 'y';
    let mut bet: i32 = 0;
    let mut double = true;
    println!("high-low game starting");

    // Game Loop: Continues until player chooses to exit the game
    while game == 'y' {
        //Deck of 52 cards created
        //Shuffle new deck every new game
        let mut deck: Deck = Deck::new_deck();
        deck.shuffle_deck();

        let mut exceed = true;
        if player.1 == 0 {
            println!("Oh wow, you're so poor, you have nothing. You poor, poor thing. Here's some pocket change.\n");
            println!("You've acquired 50 chips!");
            player.add_chips(50);
        }
        println!("You have: {} chips\n", player.1);
        while exceed {
            bet = get_bet();
            if player.check_chips(bet) && bet != 0 {
                exceed = false;
            }
        }

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
                println!(
                    "Do you think the next card will be higher or lower?\n1 = Higher, 2 = Lower\n"
                );
                choice = read!();
            }

            //Draw 2nd card
            let temp_card2 = deck.draw();
            println!("The card is a ");
            print_card(temp_card2);
            display_single(temp_card2);

            /*
            //Compare 1st and 2nd card
            if get_value(temp_card2) > get_value(temp_card1) {
                result = 1; //Card was higher
            } else if get_value(temp_card2) < get_value(temp_card1) {
                result = 2; //Card was lower
            } else if get_value(temp_card2) == get_value(temp_card1) {
                result = 3; //Card was the same
            }
            */

            match get_value(temp_card2).cmp(&(get_value(temp_card1))) {
                Ordering::Greater => result = 1,
                Ordering::Less => result = 2,
                Ordering::Equal => result = 3,
            }

            if choice == result {
                println!("You win!");
                bet *= 2;
                println!("Double or nothing? y/n\n");
                game = read!();
                if game == 'y' {
                    //double = true;
                } else {
                    //double = false;
                    println!("You've won {} chips!", bet);
                    player.add_chips(bet)
                }
            } else if result == 3 {
                println!("Card was same value, it's a draw!");
            } else {
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
    let name = match iter.next() {
        Some(t) => t.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(t) => t.unwrap(),
        None => "100".to_string(),
    };

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());

    let mut game: char = 'y';
    let mut bet: i32 = 0;
    println!("Blackjack game starting...\n");

    // Game Loop: Continues until player chooses to exit the game
    while game == 'y' {
        let mut dealer = Player::new_player();

        let mut exceed = true;
        if player.1 == 0 {
            println!("Oh wow, you're so poor, you have nothing. You poor, poor thing. Here's some pocket change.\n");
            println!("You've acquired 50 chips!");
            player.add_chips(50);
        }
        println!("You have: {} chips\n", player.1);
        while exceed {
            bet = get_bet();
            if player.check_chips(bet) && bet != 0 {
                exceed = false;
            }
        }

        //subtract bet
        player.lose_chips(bet);

        //Deck of 52 cards created
        //Shuffle new deck every new game
        let mut deck: Deck = Deck::new_deck();
        let mut hit: char;
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
            println!("Blackjack! Congrats, you win {:?} chips!", bet * 3);
            player.add_chips(bet * 3);
        } else if get_total(&player.0) != 21 && get_total(&dealer.0) == 21 {
            println!("Dealer got blackjack! You lose!")
        } else {
            println!("Want another card? y/n");
            hit = read!();

            while (get_total(&player.0) <= 21) && (hit == 'y') {
                player.add_to_hand(deck.draw());
                display_cards(&player.0);
                if get_total(&player.0) > 21 {
                    break;
                }
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
                    println!("\nDealer busted, you win {:?} chips!\n", bet * 2);
                    player.add_chips(bet * 2)
                } else {
                    match get_total(&player.0).cmp(&(get_total(&dealer.0))) {
                        Ordering::Greater => {
                            println!("\nYou win {:?} chips!\n", bet * 2);
                            player.add_chips(bet * 2)
                        }
                        Ordering::Less => println!("\nYou lose!\n"),
                        Ordering::Equal => {
                            println!("\nIt's a tie! You get your chips back\n");
                            player.add_chips(bet)
                        }
                    }
                }
            }
        }
        println!("\nDo you want to play again?\n");
        game = read!();
        player.discard_hand();
    }
}

//war!
pub fn war() {
    let mut bet: i32 = 0;

    let mut game: char = 'y';

    println!("War game starting...\n");

    //initialize / set up the game

    let mut deck: Deck = Deck::new_deck();
    deck.shuffle_deck();

    //split deck between player and dealer
    let mut player: Player = Player::new_player();
    let mut dealer: Player = Player::new_player();

    //read in from file
    let path = Path::new("save.txt");
    let display = path.display();
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut iter = reader.lines();
    let name = match iter.next() {
        Some(t) => t.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(t) => t.unwrap(),
        None => "100".to_string(),
    };

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());

    //deal cards to each person until 'deck' is empty
    while !deck.is_empty() {
        player.add_to_hand(deck.draw());
        dealer.add_to_hand(deck.draw());
    }

    //TODO: Have Reginald tell player the game is gonna start, and how it works

    //1 = dealer, 2 = player, 3 = tie

    //start game loop
    while game == 'y' {
        let mut exceed = true;
        if player.1 == 0 {
            println!("Oh wow, you're so poor, you have nothing. You poor, poor thing. Here's some pocket change.\n");
            println!("You've acquired 50 chips!");
            player.add_chips(50);
        }
        println!("You have: {} chips\n", player.1);
        while exceed {
            bet = get_bet();
            if player.check_chips(bet) && bet != 0 {
                exceed = false;
            }
        }

        //subtract bet
        player.lose_chips(bet);

        //draw
        let mut d_card = dealer.draw();
        let mut p_card = player.draw();

        //determine winner

        let mut winner = war_winner(d_card, p_card);

        while winner == 3 {
            //if tie keep looping,
            player.lose_chips(bet);
            bet += bet;
            //burn three cards

            dealer.draw();
            dealer.draw();
            dealer.draw();
            player.draw();
            player.draw();
            player.draw();

            d_card = dealer.draw();
            p_card = dealer.draw();

            winner = war_winner(d_card, p_card);
        }

        if winner == 1 {
            println!("You lose!");
        } else if winner == 2 {
            println!("You win {} chips!", bet * 2);
            player.add_chips(bet * 2);
        } else {
            println!("It's a tie!");
            player.add_chips(bet);
        }

        println!("\n\tYour Chips: {}", player.1);
        println!("Would you like to go another round? y/n");
        game = read!();
    }

    write_file(player.2, player.1);
}

///Used to handle displaying the winner - returns a number representing who won / tie
pub fn war_winner(d_card: i32, p_card: i32) -> i32 {
    //0 means forfeit tie
    //1 means dealer wins
    //2 means player wins
    //3 means its war time
    println!("   Reginald's Card");
    display_single(d_card);

    match get_value(d_card).cmp(&(get_value(p_card))) {
        Ordering::Equal => {
            //its a tie!
            println!(" .. Oh?");
            println!("   Your Card");
            display_single(p_card);

            println!("\n\n\tIt's a tie! Well.. would you like to forfeit or start a war?\n1: War\n2: Forfeit");
            let choice: char = read!();

            if choice == '2' {
                //forfeit tie - lose half
                println!("\n Ah.. That's too bad.");
                println!("\nThat means I get half");
                0
            } else {
                //war!
                println!("War! Now we burn three cards");

                3
            }
        }
        Ordering::Greater => {
            //dealer wins
            println!(" Dealer Wins!");
            println!("   Your Card");
            display_single(p_card);
            1
        }
        Ordering::Less => {
            //player wins
            println!(" You Win!");
            println!("   Your Card");
            display_single(p_card);
            2
        }
    }
}

///Red Dog Poker
pub fn red_dog_poker() {
    println!("Red Dog Poker Starting..");

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
    let name = match iter.next() {
        Some(t) => t.unwrap(),
        None => "Player".to_string(),
    };
    let chips = match iter.next() {
        Some(t) => t.unwrap(),
        None => "100".to_string(),
    };

    player.set_name(name);
    player.add_chips(chips.parse::<i32>().unwrap());

    let mut deck: Deck = Deck::new_deck();

    let mut bet_amount: i32 = 10;

    println!("Would you like to hear the rules? y/n?");
    let mut game: char = read!();

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
    }

    game = 'y';
    print!("\n------ Let's Play Red Dog Poker! ------\n");

    deck.shuffle_deck();

    while game == 'y' {
        let mut exceed = true;

        println!("You have: {} chips\n", player.1);

        if player.1 == 0 {
            println!("Oh wow, you're so poor, you have nothing. You poor, poor thing. Here's some pocket change.\n");
            println!("You've acquired 50 chips!");
            player.add_chips(50);
        }
        println!("You have: {} chips\n", player.1);

        while exceed {
            println!("\nWhat would you like to bet? (Default 10)");
            let input: String = read!();
            match input.parse::<i32>() {
                Err(_e) => exceed = true,
                Ok(f) => bet_amount = f,
            };
            if player.check_chips(bet_amount) {
                exceed = false;
            }
        }

        player.lose_chips(bet_amount);

        //deal player two cards
        player.add_to_hand(deck.draw());
        player.add_to_hand(deck.draw());
        //display hand
        println!("\n\tYour Hand:");
        display_cards(&player.0);

        //if first two cards are consecutive, push

        if player.0[0] + 1 == player.0[1] || player.0[0] - 1 == player.0[1] {
            println!("It's a push, so nothing happens");
            player.add_chips(bet_amount);
        } else {
            //ask if they'd like to double down, or call
            println!("Would you like to: \n1: double down\n2: call");
            let button: char = read!();

            if button == '1' {
                //double down
                player.lose_chips(bet_amount);
                player.add_to_hand(deck.draw());
                print!("\n\tBetting: {} chips", bet_amount + bet_amount);
                println!("\n\tYour Hand:");
                display_cards(&player.0);
                let hand: Vec<i32> = [player.0[0], player.0[1], player.0[2]].to_vec();

                //process payout
                let mut payout = bet_amount + rdp_payout(hand, bet_amount);
                if payout == 0 {
                    //this means you lost
                    println!("ooh, Bust. You lose those chips");
                } else {
                    //add another bet_amount to payout since it was doubled
                    payout += bet_amount;
                    println!("You've earned {} chips", payout);
                    player.add_chips(payout);
                }
            }
        }

        player.discard_hand();
        println!("\nContinue? y/n");
        game = read!();
    }
    //game == 'n' so end game
    write_file(player.2, player.1);
}

///takes the player object and checks its hand to determine what the payout is
pub fn rdp_payout(hand: Vec<i32>, bet_amount: i32) -> i32 {
    //should only be called when player has 3 cards
    let a = hand[0];
    let b = hand[1];
    let c = hand[2];

    //not same, not in spread
    if a != b && c > cmp::max(a, b) && c < cmp::min(a, b) {
        -bet_amount
    }
    //all 3 same
    else if a == b && c == b {
        bet_amount + 11
    }
    //spread
    else {
        let spread = cmp::max(a, b) - cmp::min(a, b);

        //only one possible card ex: if a = 11 and b = 9, c has to be 10 to win
        if spread == 2 {
            bet_amount + 5
        } else if spread == 3 {
            bet_amount + 4
        } else if spread == 4 {
            bet_amount + 2
        } else {
            bet_amount + 1
        }
    }
}

///function to test all test-able functions of Rusty Casino
pub fn test() {
    let _result: bool;
    println!("Testing started");

    let mut deck: Deck = Deck::new_deck();
    assert_eq!(
        52,
        deck.len(),
        "Checking if the newly created deck has 52 cards"
    );

    println!("\n\tShowing unshuffled deck:");
    deck.show_deck();
    println!("\n\tShowing shuffled deck:");
    deck.shuffle_deck();
    deck.show_deck();

    let mut empty_deck: Deck = Deck::empty_deck();
    assert_eq!(
        empty_deck.is_empty(),
        true,
        "Checking if empty deck is empty"
    );

    let card = deck.draw();
    //place card in empty deck
    empty_deck.add_card(card);
    assert_eq!(
        empty_deck.is_empty(),
        false,
        "make sure empty deck isnt empty anymore"
    );

    let mut player: Player = Player::new_player();
    assert_eq!(
        true,
        player.0.is_empty(),
        "Check if new player is empty (it should be empty)"
    );
    println!("adding card to player's hand");
    player.add_to_hand(deck.draw());
    assert_eq!(
        false,
        player.0.is_empty(),
        "Checking if player isn't empty (we added 1 card)"
    );

    player.set_name("Test".to_string());
    assert_eq!("Test", player.2, "Testing set_name()");

    player.add_chips(10);
    assert_eq!(10, player.1, "testing add_chips()");

    player.lose_chips(9);
    assert_eq!(1, player.1, "testing lose_chips()");

    player.discard_hand();
    println!("emptying hand");
    assert_eq!(player.0.is_empty(), true, "testing discard_hand()");

    println!("\n\n\n\tTEST SUCCESSFUL!");
}
