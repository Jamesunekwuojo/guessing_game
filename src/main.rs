use std::io; // imporing io from the rust standard prelude
use rand::Rng; // brought Rng trait to scope
use std::cmp::Ordering;
//First we add another use statement, bringing a type called std::cmp::Ordering into scope from the standard library. The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.




fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generating a random number between 1 and  100

    println!("Secret number {}", secret_number);


    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // it returns  a new instnace of a  string

        io::stdin().read_line(&mut guess).expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // Shadowing original guess and trimming whitespce begiining and end 

        //The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. I 

        // The expect method is used for handling errros when the result from the 'parse' method is not succesfyl

        // We switch from an expect call to a match expression to move from crashing on an error to handling the error. Remember that parse returns a Result type and Result is an enum that has the variants Ok and Err. We’re using a match expression here, as we did with the Ordering result of the cmp method.

     

       match guess.cmp(&secret_number) {
          Ordering::Less =>  println!("Too small"),
          Ordering::Greater => println!("Too big!"),

          Ordering::Equal => {
            println!("You win");
            break;
          }
        }
    }

 

    // By default in Rust variablrs are immutable for you to make it mutable, you need to introduce.. 'mut'

    // The :: synthax in the ::new line indicates  that new is an associated function of the String type.  An associated function is a function that’s implemented on a type, in this case String

    // Crate is a collection of Rust source code files.
    // To generate random nomber we are going to be making use of the rand crate libary
    // The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own. 

    // Cartgo understand semantic versioning...

    // rand = "0.8.5" it's the shorthand for   ^0.8.5,  major, minor, and patch.  which means any version that is at least 0.8.5 but below 0.9.0.

    // When we include an external dependency e.g the rand, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.


    // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.

    //A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. 


}