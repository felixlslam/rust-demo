use std::{io, sync::{atomic::AtomicU32, Arc}, thread::{self, sleep}, time::Duration};

use rand::Rng; //Rng is a trait that needs to be in-scope so that the method from this trait can be used.

fn main() {
    //As secret_number needs to be accessed in both the main thread and the spawned thread, it needs to be wrapped inside Arc (Asynchronous Reference Count)
    //Besides, the secret_number needs to be mutated inside the spawned thread, Arc doesn't support mutation of the inner value, we needs to use AtomicU32 to store the u32 value which allows mutation in a thread-safe manner
    let secret_number: Arc<AtomicU32> = Arc::new(AtomicU32::new(rand::thread_rng().gen_range(0..=100))); 
    println!("The secret number is {}", secret_number.load(std::sync::atomic::Ordering::Relaxed));

    let secret_number_clone = secret_number.clone(); //To share an Arc, between different threads, we need to "clone" the Arc before the passing to a thread

    thread::spawn(move || {
        loop {
            sleep(Duration::from_secs(5));
            //The value wrapped by an Arc is automatically dereferenced (deref) that's why we can directly use the load() and store() method of AtomicU32 below
            secret_number_clone.store(rand::thread_rng().gen_range(0..=100), std::sync::atomic::Ordering::Relaxed);
            println!("**** New secret number is {} *****", secret_number_clone.load(std::sync::atomic::Ordering::Relaxed));
        }
    });

    loop {
        let guess = get_user_guess();
        println!("You've guessed {guess}");
        match guess.cmp(&secret_number.load(std::sync::atomic::Ordering::Relaxed)) {
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


