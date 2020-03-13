#[cfg(test)]
mod tests{

    use high_low::deck::*;

    #[test]
    fn test_deck(){
        let  deck:Deck = Deck::new_deck();
        assert_eq!(52, deck.0.len());

        let empty_deck:Deck = Deck::empty_deck();
        assert_eq!(deck.0.is_empty(), true);

    }


}
