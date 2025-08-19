use std::cmp::Ordering; // Enum: less, greater, equal
use std::io; // input/output library, from the standard library

use rand::Rng; // random number generating functions

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // thread_rng(): seeded by the local OS

    println!("The secret number is: {secret_number}");

    loop {
        println!("Input your guess.");

        let mut guess = String::new(); // generates an empty string and assigns to the mutable `guess` variable; the :: indicates that `new` is an associated function of the String type

        io::stdin()  // calling the `stdin()` function from the io module, which allows us to handle user input. Alternatively, we could write this function as std::io::stdin - if we hadn't imported io already.
            .read_line(&mut guess)  // calls the read_line method from stdin() to get input from the user. &mut (& indicating a reference pointer) guess is passed to read_line to tell it what string to store the user input. The string arg needs to be mutalbe so the method can change the string's content
            .expect("failed to read line"); // since read_line produces a Result (which is an enum; or a type that can be one of multiple possible states), we need to handle the variant. If the Result is an `Err` .expect is called which will produce this message to the user. But if Result is `Ok` .expect will just return the value alongside the Ok result.

        let guess: u32 = match guess.trim().parse() { // parse returns a Result type (an enum; functionality noted above) and we `match` it:
            Ok(num) => num,
            Err(_) => continue, // _ is catch-all because we don't care about what kind of error it is; we'll just have them input a new guess
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }
    }
}
