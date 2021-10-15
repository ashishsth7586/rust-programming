// brings the io (input/output) library into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please enter your guess: ");

        // The :: syntax in the ::new line indicates
        // that new is an associated function of the String type.
        // An associated function is implemented on a type, in this case String.
        let mut guess = String::new();

        // The job of read_line is to take whatever the user types
        // into standard input and append that into a string
        // (without overwriting its contents), so it takes that string as an argument.
        // The string argument needs to be mutable so the method can change the string’s
        // content by adding the user input.

        // The & indicates that this argument is a reference,
        // which gives you a way to let multiple parts of your code access
        // one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // We create a variable named guess. But wait, doesn’t the program already have a
        // variable named guess? It does, but Rust allows us to shadow the previous value
        // of guess with a new one. This feature is often used in situations in which you
        // want to convert a value from one type to another type. Shadowing lets us reuse
        // the guess variable name rather than forcing us to create two unique variables,
        // such as guess_str and guess for example.
        // The trim method on a String instance will eliminate any whitespace at the beginning and end.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}, Try Again!!!", err);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // if guess == secret_number {
        //     println!("Equal! you win!")
        // } else {
        //     println!("You loose")
        // }
    }
}
