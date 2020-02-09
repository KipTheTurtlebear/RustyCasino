extern crate deck;

fn main() {
    let deck = deck::Deck::new_deck();
    deck.len();
    deck.show_deck();
    deck.shuffle();
    deck.show_deck();
}