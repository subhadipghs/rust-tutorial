
use std::io;

fn main() {
    println!("enter ur guess: ");
    // store the user guess 
    let mut guess = String::new();

    // read from the input
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input");

    println!("your guess is {}", guess);
}
