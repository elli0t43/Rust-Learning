// we can import files with "mod" keyword
mod print;

// our main function
fn main() {
    println!("Hello, world!");
    print::run(); // calling the run function from print.rs file
}
