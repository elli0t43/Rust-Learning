// we can import modules with "mod" keyword
mod print;
mod variables;

// our main function
// The fn syntax declares a new function, the parentheses,
// (), indicate there are no parameters,
// and the curly bracket,
// {, starts the body of the function.
fn main() {
    println!("Hello, world!"); // do keep in mind that println! isn't a function its a macro.
    print::run(); // calling the run function from print.rs file
    variables::var_run();
}
