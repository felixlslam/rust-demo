use std::io;

fn main() {
    let guess = get_user_guess();
    println!("You've guessed {guess}");
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
        // guess variable is automatically here
    }
}


