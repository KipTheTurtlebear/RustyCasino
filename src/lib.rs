mod high_low;
use text_io::read;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

pub fn Rusty_Casino(){
    let mut choice = 0;

    println!("\tPlease enter your name:\n");
    let mut name: String = read!();
    let mut textfile: String = name.clone();
    textfile.push_str(".txt");
    // Create file and save path
    let path = Path::new(&textfile);
    let display = path.display();

    // Open the file in write-mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(name.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Your name is: {:?}", name),
    }

    while choice != 9 {
        println!("Select which game you'd like to play:\n1: High Low\n9: Leave Casino\n");
        choice = read!();
        if choice == 1 {
            high_low::high_low();
        }
    }

}
