use std::io;              // pulling in input/output library from standard
use rand::Rng;            // pulling in rng library
use std::cmp::Ordering;   // standard library for comparing

// declares main function
fn main() {
    // macro to print to screen
    println!("Guess the number!");

    // generating random number
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        // creates a mutable variable bound to a String type
        let mut guess = String::new();

        // using input function to read line from user
        // &mut guess is a reference to guess variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");  // handling errors

        // change guess from string to unsigned-32 bit int
        // match handles user error if not given a number
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // giving the guess to a string argument
        println!("You guessed: {}", guess);

        // comparing guess to secret
        // breaks out of loop if equal to secret
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
