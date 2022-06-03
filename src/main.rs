use std::io;

fn main() {
    // ! === macro
    println!("Guess the Number!");
    println!("Write your tip!.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite).expect("Failed to read input!");

    println!("You said: {}", palpite);
}
