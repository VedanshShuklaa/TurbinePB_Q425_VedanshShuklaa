use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); //Had to : u32 cuz apparently cmp can't compare u32 with other num types???

    println!("The secret number is {secret_number}");

    println!("Please input your guess :");

    let mut guess = String::new();

    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }   
    }
}