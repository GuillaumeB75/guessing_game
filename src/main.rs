
extern crate colored;

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}","Guess the number!".blue());
    let max_guess = 100;
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
        match guess.cmp(&max_guess) {
            Ordering::Greater => println!("The secret number is between 1 and 100!"),
            Ordering::Less => {
                continue;
            }
            Ordering::Equal => {
                continue;
            }
        }
    }
}
