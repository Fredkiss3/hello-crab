use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    println!("Guess the number! ");
    // this variable here "secret_number" is infered based on usage
    // by default secret_number is of type i32, but since we used a u32 number to compare it to,
    // it is infered by the compiler as a u32
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess: String;

    // loop for looping indefinitely
    loop {
        guess = String::new();
        println!("Please input your guess :");

        io::stdin()
            .read_line(&mut guess)
            // expect crashes the program is an error occurs while running the program
            .expect("Failed to read line");

        // Notes :
        // 1- String.parse is a generic method than can parse into any type
        // that implement a `FromStr` method, but here since we declared guess as a u32 int,
        // Parse is infered as u32, but the behavior of the method is changed to parse into a u32 🤯
        //
        // 2- Here we redeclare `guess`, seems like this is possible to do in rust, but
        // i think this could lead to unwanted bugs
        //
        // 3 - you manually handle function that can throw an error with a match expression
        // a function that can throw an Error returns a `Result` type which an enum with two values (called `variants`) :
        // => Ok(return_type)
        // => Err(error_type)
        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("❌ Please type a valid number ❌");
                continue;
            }
        };
        println!("You guessed: {guess}");

        // A match is like a switch expression
        // But you have to declare all the cases
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!😢"),
            Ordering::Greater => println!("Too big!🙅‍♂️"),
            Ordering::Equal => {
                println!("You win! 😎");
                break;
            }
        }
    }
}
