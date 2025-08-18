use std::io; // input/output library, from the standard library

fn main() {
    println!("Guess the number!");

    println!("Input your guess.");

    let mut guess = String::new(); // generates an empty string and assigns to the mutable `guess` variable; the :: indicates that `new` is an associated function of the String type

    io::stdin()  // calling the `stdin()` function from the io module, which allows us to handle user input. Alternatively, we could write this function as std::io::stdin - if we hadn't imported io already.
        .read_line(&mut guess)  // calls the read_line method from stdin() to get input from the user. &mut (& indicating a reference pointer) guess is passed to read_line to tell it what string to store the user input. The string arg needs to be mutalbe so the method can change the string's content
        .expect("failed to read line"); // since read_line produces a Result (which is an enum; or a type that can be one of multiple possible states), we need to handle the variant. If the Result is an `Err` .expect is called which will produce this message to the user. But if Result is `Ok` .expect will just return the value alongside the Ok result.

    println!("You guessed: {guess}");
}
