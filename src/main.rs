// we can import files with "mod" keyword
mod print;
mod variables;

// our main function
fn main() {
    println!("Hello, world!"); // do keep in mind that println! isn't a function its a macro.
    print::run(); // calling the run function from print.rs file
    variables::var_run();
}
