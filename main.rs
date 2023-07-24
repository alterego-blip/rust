use std::io;
use rand::Rng; //RNG trait
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // An infinite loop
    loop { 

        println!("Enter a number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error occured"); // Just to calm the compiler

        //Example of shadowing in rust
        //Also, error handling without panicking using match statement.
        let guess: u32 = match guess.trim().parse() {            //.expect("Enter a number!"); 
            Ok(num) => num,
            Err(_) => continue
        }; // Requires ; as this is a declaration statement.
        //When Result value ok, it returns the parsed u32 integer.

        // Value of u32 makes cmp infer type of secret_number as u32
        match guess.cmp(&secret_number) {                                           
            // The leftsides are called arms
            Ordering::Less => println!("The number you guessed is lower!"), 
            Ordering::Greater => println!("The number you guessed is greater!"),
            Ordering::Equal => {
                println!("You guessed {guess}");
                break; // Breaking the loop ends the program.

            }
        } // Doesn't require ; as this is just a match "expression"
    }
}
