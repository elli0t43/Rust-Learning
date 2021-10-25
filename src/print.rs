pub fn run() {
    // print to console
    // print: It is the name of a macro defined in the Rust standard library
    // !: It specifies that the preceding name indicates a macro.
    // Without such a symbol, print would instead indicate a function.
    // There is no such function in the Rust standard library, and so you would get compilation error
    println!("Hello from print.rs");
}
