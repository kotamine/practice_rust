use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Input your guess;");
        
        // create a mutable variable to store the user input
        let mut guess = String::new();
        
        // read the user input and store it in the guess variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // convert the user input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!"); 
                continue;
            },
        };

        println!("You guessed: {guess}");

        // compare the user input with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        };
    }
}
