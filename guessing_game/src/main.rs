use std::io;    // to obtain user input and then print the result as output "io" input/output libray into scope, "io" library comes from the standard library, known as std:

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess= String::new();   // new varialbe that mutable and has empty string value

    io::stdin()
        .read_line(&mut guess)      // & == references, so mutate variables
        .expect("Failed to red line");                  // handling potential Failure with the Result type

    println!("You guessed: {guess}");
}

/*
    Variables!
    let apples = 5;         immutable
    let mut bananas = 5;    mutable
*/