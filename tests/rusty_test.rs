#[cfg(test)]
mod tests{
    #[test]
    fn test_deck(){
        let deck:Deck = Deck::new_deck();
        assert_eq!(52, deck.0.len());

        let empty_deck:Deck = Deck::empty_deck();
        assert_eq!(deck.0.is_empty(), true);
        
       
        let card = deck.draw();
        let temp = card;

        empty_deck.add_card(card);
        assert_eq!(temp, empty_deck.draw());
    }

    #[test]
    fn test_shuffle(){
        let deck:Deck = Deck::new_deck();

        println!("\n\tShowing original deck:");
        deck.show_deck();

        println!("\n\tShowing shuffled deck:");
        deck.shuffle_deck();
        deck.show_deck();
    }
}
