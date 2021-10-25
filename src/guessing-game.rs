// To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope.
// The io library comes from the standard library (which is known as std):
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    //let statement, which is used to create a variable
    /* The :: syntax in the ::new line indicates that new is an associated function of the String type.
    This new function creates a new */
    let mut guess = String::new();
    
    // Now we’ll call the stdin function from the io module
    /* If we hadn’t put the use std::io line at the beginning of the program,
    we could have written this function call as std::io::stdin */   
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        .

    println!("You guessed: {}", guess);
}
