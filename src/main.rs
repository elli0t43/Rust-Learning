// we can import modules with "mod" keyword
mod contstants;
mod print;
mod scalar_types;
mod variables;
mod number_guessing_game;
mod assert_eq_macro;

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
    //scalar_types::integer_types_run();
    //scalar_types::integer_overflow_run();
    //scalar_types::float_types_run();
    //scalar_types::number_seperator_run();
    //scalar_types::boolean_types_run();
    //scalar_types::character_types_run();
    //contstants::constants_types_run();
    //contstants::shadowing_var_char_run();
    number_guessing_game::number_guessing_game_run();
    assert_eq_macro::assert_eq_macro_run();
}
