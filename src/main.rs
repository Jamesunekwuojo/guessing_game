use std::io;

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

}