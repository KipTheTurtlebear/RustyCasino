mod high_low;
use text_io::read;

pub fn Rusty_Casino(){
    let mut choice = 0;
    while choice != 9 {
        println!("Select which game you'd like to play:");
        choice = read!();
        if choice == 1 {
            high_low::high_low();
        }
    }

}
