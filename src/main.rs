use std::io;

fn main() {
    println!("Guess the number!\n");
    println!("Enter a number: ")

    let mut guess = String::new()

    io::stdin().readline(&mut guess).expect("Failed to read line.");
    println!("You guessed: {guess}!")
}