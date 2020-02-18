mod high_low;
use text_io::read;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

pub fn Rusty_Casino(){
    let mut choice = 0;
    println!("\n\n\n\n");
    println!("████████╗██╗  ██╗███████╗    ██████╗ ██╗   ██╗███████╗████████╗██╗   ██╗     ██████╗ █████╗ ███████╗██╗███╗   ██╗ ██████╗ ");
    println!("╚══██╔══╝██║  ██║██╔════╝    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝╚██╗ ██╔╝    ██╔════╝██╔══██╗██╔════╝██║████╗  ██║██╔═══██╗");
    println!("   ██║   ███████║█████╗      ██████╔╝██║   ██║███████╗   ██║    ╚████╔╝     ██║     ███████║███████╗██║██╔██╗ ██║██║   ██║");
    println!("   ██║   ██╔══██║██╔══╝      ██╔══██╗██║   ██║╚════██║   ██║     ╚██╔╝      ██║     ██╔══██║╚════██║██║██║╚██╗██║██║   ██║");
    println!("   ██║   ██║  ██║███████╗    ██║  ██║╚██████╔╝███████║   ██║      ██║       ╚██████╗██║  ██║███████║██║██║ ╚████║╚██████╔╝");
    println!("   ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝      ╚═╝        ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝ ");
    println!("\n\n\n\n");
    println!("\tAs you walk through the doors, you're approached by a dashing gentleman.");
    println!("\t\"Hello, my name is Reginald. It will be my pleasure to be your aide today\"");
    println!("\t\"What may I call you today?\"\n");
    println!("\nPlease enter your name below:\n");
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
        Ok(_) => println!("\n\n\"Ah, {:?}, what a wonderful name.\"\n", name),
    }
    println!("\"There was a {:?} here the other day. Lost his house. I'm sure you'll do better though, eh?\"\n", name);

    while choice != 9 {
        println!("Before you are several numbered doors. Which would you like to explore?:\n1: High Low\n9: Leave Casino\n");
        choice = read!();
        if choice == 1 {
            high_low::high_low();
        }
    }

}
