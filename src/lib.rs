mod high_low;

pub fn Rusty_Casino(){
    let choice = 0;
    while choice != 9 {
        println!("Select which game you'd like to play:");
        high_low::high_low();
    }

}
