use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let my_guess = rand::thread_rng().gen_range(1, 11);

    let quit_condition = "quit";

    loop {
        println!("Guess...");
        println!("Enter your guess");

        let mut your_guess = String::new();

        io::stdin().read_line(&mut your_guess)
            .expect("Input Failure");

        let your_guess: u32 = match your_guess.trim().parse() {
            Ok(input) => input,
            Err(_) => match your_guess.trim() == quit_condition {
                true => break,
                false => {
                    println!("You have to enter a number");
                    continue;
                }
            }
        };


        println!("Your Guess:{}", your_guess);

        match your_guess.cmp(&my_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}