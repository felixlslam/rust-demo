use std::io;

fn main() {
    get_user_guess();
}

fn get_user_guess() -> u32 {
    loop {
        let mut guess = String::new();
        println!("Please guess the number");
        io::stdin().read_line(&mut guess).expect("Cannot read user input");
        match guess.trim().parse() {
            Ok(num) => return num,
            Err(err) => {
                println!("Error : {err}");
            }
        }
    }
}


