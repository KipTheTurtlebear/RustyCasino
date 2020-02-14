//player module

#[derive(Debug, Default, Clone)]
pub struct Player(Vec<i32>);



impl Player{

    ///Create the player
    pub fn new_player() -> Self{

        //don't really know if we need this
        Player((0..0).collect())
    }

    ///Add card to player's hand
    pub fn add_to_hand(mut self, card: i32) -> Self{

        //push to hand
        self.0.push(card);
        self
    }
}


