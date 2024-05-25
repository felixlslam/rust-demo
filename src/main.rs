use std::io;

use rand::Rng; //Rng is a trait that needs to be in-scope so that the method from this trait can be used.

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    println!("The secret number is {secret_number}");

    loop {
        let guess = get_user_guess();
        println!("You've guessed {guess}");
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_user_guess() -> u32 {
    loop {
        let mut guess = String::new(); // guess string variable is allocated here
        println!("Please guess the number");
        io::stdin().read_line(&mut guess).expect("Cannot read user input"); // pass guess variable as a mutable reference , a borrow happens here
        match guess.trim().parse() {
            Ok(num) => return num, // compiler knows the guess string is parsed to u32 by type inference
            Err(err) => {
                println!("Error : {err}");
            }
        }
        // guess variable is automatically dropped here
    }
}


