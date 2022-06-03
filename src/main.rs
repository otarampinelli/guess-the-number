extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // ! === macro
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Write your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read input! \n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You said: {} \n", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Very low guess! \n"),
            Ordering::Greater => println!("Very high guess! \n"),
            Ordering::Equal => {
                println!("You got the guess right! \n");
                break;
            }
        }
    }
}
