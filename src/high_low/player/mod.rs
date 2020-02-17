//player module

#[derive(Debug, Default, Clone)]
pub struct Player(Vec<i32>, i32); //Hand of cards, chips



impl Player{

    ///Create the player
    pub fn new_player() -> Self{
        //don't really know if we need this
        // Player((0..0).collect())
        new_player(Vec::new(), 0)
    }

    ///Add card to player's hand
    pub fn add_to_hand(mut self, card: i32) -> Self{

        //push to hand
        self.0.push(card);
        self
    }

    pub fn add_chips(mut self, chips: i32) -> Self{
       match self.1.checked_add(chips) {
           Some(v) => {
               self.1 = v;
           }
           None => {
               println!("You're OVERFLOWING with chips");
           }
       }
        self
    }
}


