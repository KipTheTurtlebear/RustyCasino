
# Rusty Casino README

#### Team members: Princess Kim (prikim@pdx.edu) & Michael Jenkins ([mtj4@pdx.edu](mailto:mtj4@pdx.edu))

  

# What is Rusty Casino?

Rusty Casino is a Library filled with 4 different card games built with varying modules that imitate player objects and a deck. Cards and decks are represented as i32 values (vector of i32 for multiple) and players have a vector of i32 representing their hand, chips, and their name. Player progress is preserved via writing and loading to a file. Player’s are able to bet chips and go between games increasing their earnings, or possibly losing it all. It can be run with a simple `cargo run` command

  

# How was work split

  

Independent work

-   War & Red Dog Poker - Princess
    
-   High_low & Blackjack - Michael
    

  

Collaborative

-   `Deck.rs`
    
-   `Player.rs`
    

  

Our git : [https://github.com/KipTheTurtlebear/RustyCasino/branches](https://github.com/KipTheTurtlebear/RustyCasino/branches)

  

# Struggles/Roadblocks and Their Remedies

One of the early struggles was figuring out what the architecture would look like - how were we going to represent cards and players, and how would we create different card games? Figuring out how the modules would be separated was fairly easy. We knew we wanted a Deck, Player, and different games, but the actual directory placement for the modules and referencing them in different files was confusing, but eventually got it to work. This specific issue was seemingly solved, but we had to implement the rest of our games in the high_low directory (the first game that was implemented), and towards the end when we needed to create some tests, It was a struggle to get the test/test.rs to recognize the modules. This was ‘fixed’ by creating a test function to be called from ``rusty_casino()``. We had quite a few warnings piling up before fixing them, and ``cargo clippy`` wasn’t run until most of the games were finished. One of the more confusing warnings to fix was Rust wanting us to change the chain of if statements into a match

[https://github.com/rust-lang/rust-clippy/issues/4531](https://github.com/rust-lang/rust-clippy/issues/4531)

  

# How did we test

As stated earlier, a test function was created as a hidden option in ``rusty_casino()``. The user is prompted to choose from 5 different options (4 games, or leave the casino). If you enter 0, the test function will run with several ``assert_eq!()`` testing various functionalities implemented into the project. It’ll test the deck module functions ``new_deck()`` ``empty_deck()`` and things like ``draw()`` and making sure that the deck manipulations go correctly. Same with the player module. As for testing to see if writing and loading to the file works, it was tested via trial and error. Each game was created without betting and chips being processed. Once the games were mostly done, betting and the likes were implemented, and then lastly loading and writing to the file was put. Once that was done, we just ran the program and play tested the games, while ensuring the chips we were earning/losing stayed the same no matter what game we were playing. Games display your chips and how much you’re betting, as well as handling the case of you wanting to bet more than what you have. If you had 10 chips left, and you wanted to bet 15, it wouldn’t let you. A pity system was implemented so that if a player ran out of chips completely, the dealer would feel sorry for them and give them 50 chips.

![](https://lh6.googleusercontent.com/6Xc-jrkyQ6LLhFM-SrBEaRUeFC9VjKMsJigFf8pK4I4STZ4lht_jkQ7yQ-YO1rudf2xCgf4KUGvuppAcUJfzu6I5juqS2wSS4O1xfTuX3ctYVWHUFnDjVbT-GzCiAXoLC4Mb0-zY)

  

# Our Overall Feelings

Having more time to refine the game would be nice. We originally thought that encrypting the file would be nice as to deter people from editing their own save file, but was unable to do so. Having a better way of displaying the game, like in a separate window with interactable objects would’ve been nice as well. It was great to see how Rusty Casino evolved over time and now having a finished program that is ‘playable’ is nice. We had the common problem of thinking too grand to start, and having to decide which features were critical or more important than others. Once it was all said and done, I think we're both pretty satisfied with the end result.
