use std::io;
use std::cmp::Ordering;
use rand::Rng;

//Simple program where you can use Binary Search to guess the number in just a few guesses

fn main() {
    println!("Guess the Number in 7 guesses! (The number lies between 1 and 100)");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut guess_count = 0;

    loop {
        guess_count += 1;
        if guess_count == 7 { break; }
        println!("Please input guess {guess_count}:");

        let mut guess = String::new();

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
