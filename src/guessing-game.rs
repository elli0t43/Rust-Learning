// To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope.
// The io library comes from the standard library (which is known as std):
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    //let statement, which is used to create a variable
    let mut guess = String::new(); // also this is muteable, because of that mut flag

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
