pub fn run() {
    // print to console
    // print: It is the name of a macro defined in the Rust standard library
    // !: It specifies that the preceding name indicates a macro.
    // Without such a symbol, print would instead indicate a function.
    // There is no such function in the Rust standard library, and so you would get compilation error
    println!(); // prints just a newline
    println!("hello "); //prints hello
    println!("my {} is elli0t43", "name"); //prints my name is elli0t43
                                           /* In the above case, The println! macro takes two arguments −
                                            1. A special syntax { }, which is the placeholder
                                            2. The variable name or a constant,
                                           The placeholder will be replaced by the variable’s value*/
}
