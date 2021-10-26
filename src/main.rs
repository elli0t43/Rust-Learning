// we can import modules with "mod" keyword
mod print;
mod scalar_types;
mod variables;

// our main function
// The fn syntax declares a new function, the parentheses,
// (), indicate there are no parameters,
// and the curly bracket,
// {, starts the body of the function.

// REMEMBER TO USE SNAKE CASE WITH LONG VARIABLE NAMES. CAMEL and PASCAL CASE WON'T WORK
fn main() {
    println!("Hello, world!"); // do keep in mind that println! isn't a function its a macro.
    print::run(); // calling the run function from print.rs file
                  // variables::var_run();
    scalar_types::integer_types_run();
    scalar_types::integer_overflow_run();
    scalar_types::float_types_run();
    scalar_types::number_seperator_run();
    scalar_types::boolean_types_run();
    scalar_types::character_types_run();
}
