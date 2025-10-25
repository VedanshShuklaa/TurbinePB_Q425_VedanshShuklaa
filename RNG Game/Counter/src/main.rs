use std::io;

fn main() {
    let mut count: i32 = 0;

    println!("Counter started at {count}.");
    println!("Type '+' to increase, '-' to decrease, or 'q' to quit.");

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "+" {
            count += 1;
            println!("Count: {count}");
        } else if input == "-" {
            count -= 1;
            println!("Count: {count}");
        } else if input == "q" {
            println!("Done. Final count: {count}");
            break;
        } else {
            println!("Not sure what that means. Try '+', '-', or 'q'.");
        }
    }
}
