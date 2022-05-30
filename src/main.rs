use std::io;

fn main() {
    println!("Guess the number!hello");

    println!("Please input your guess and wait you know.");

    let mut guess = String::new();
    let mut x = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line x");
    println!("You guessed: {} and {}", guess, x);
}
