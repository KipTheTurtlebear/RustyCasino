mod high_low;
use crate::high_low::player::*;

//use std::fs::File;
//use std::fs::OpenOptions;
//use std::io::prelude::*;
//use std::path::Path;
use text_io::read;

pub fn rusty_casino() {
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
    println!("\t\"What may I call you?\"\n");
    println!("\nPlease enter your name below:\n");
    let name: String = read!();
    let mut chips = 100;
    /*
        let mut textfile: String = name.clone();
        textfile.push_str(".txt");

        // Create file and save path
        let path = Path::new("save.txt");
        let display = path.display();

        // Open the file in write-mode
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        match file.write_all(name.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("\n\n\t\"Ah, {:?}, what a wonderful name.\"\n", name),
        }

        let mut file = match OpenOptions::new().append(true).open(path) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
            Ok(_) => file,
        };
    */
    if &name == "Bart" {
        println!("\t\"Oh heavens, it's you! The creators of this fine establishment would like to give you a bri-, I mean, a gift\"\n");
        //    write!(file, "\n{}", 10000);
        chips = 10000;
        println!("You have received 10,000 chips!\n");
    } else {
        println!("\t\"There was a {:?} here the other day. Lost his house. I'm sure you'll do better though, eh?\"\n", name);
        //  write!(file, "\n{}", 100);
        println!("\t\"Here's some chips to get you started tonight\"");
    }
    write_file(name, chips);
    while choice != 9 {
        println!("Before you are several numbered doors. Which would you like to explore?:\n1: High Low\n2: Blackjack\n3: War\n4: Red Dog Poker\n9: Leave Casino\n");
        choice = read!();
        if choice == 1 {
            high_low::high_low();
        } else if choice == 2 {
            high_low::blackjack();
        } else if choice == 3 {
            high_low::war();
        } else if choice == 4 {
            high_low::red_dog_poker();
        } else if choice == 0 {
            high_low::test();
        }
    }
}
