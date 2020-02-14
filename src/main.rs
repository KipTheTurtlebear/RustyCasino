mod deck;
use crate::deck::*;

fn main() {
    let mut deck: Deck = Deck::new_deck();
    println!("------ {} cards in deck ------", deck.len());
    deck.show_deck();
    println!("------ Deck is being shuffled ------");
    deck.shuffle_deck();
    deck.show_deck();

    println!("---- Drawing 1 card ----");
    println!("you drew:");
    print_card(deck.draw());
    println!("---- new deck ----");
    deck.show_deck();
}
