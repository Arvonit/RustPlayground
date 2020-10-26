use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run() {
    // `gen_range()` is inclusive on the lower bound but exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");
    // println!("BTW the secret number is {}. Don't tell anyone!", secret_number);
    
    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
            
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small."),
            Ordering::Greater => println!("Your guess was too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
