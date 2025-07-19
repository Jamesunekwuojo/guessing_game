use std::io; // imporing io from the rust standard prelude

fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new(); // it returns  a new instnace of a  string

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");

    let x = 5;
    let y = 10;

    println!("x= {x} and y + x = {} ", y +2);

    // By default in Rust variablrs are immutable for you to make it mutable, you need to introduce.. 'mut'

    // The :: synthax in the ::new line indicates  that new is an associated function of the String type.  An associated function is a function that’s implemented on a type, in this case String

    // Crate is a collection of Rust source code files.
    // To generate random nomber we are going to be making use of the rand crate libary
    // The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own. 

    // Cartgo understand semantic versioning...

    // rand = "0.8.5" it's the shorthand for   ^0.8.5,  major, minor, and patch.  which means any version that is at least 0.8.5 but below 0.9.0.

    // When we include an external dependency e.g the rand, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.


}