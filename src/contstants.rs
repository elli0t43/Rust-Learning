/*
Constants represent values that cannot be changed. If you declare a constant then there is no way its value changes. The keyword for using constants is const. Constants must be explicitly typed. Following is the syntax to declare a constant.

`const VARIABLE_NAME:dataType = value;`

Rust Constant Naming Convention
The naming convention for Constants are similar to that of variables. All characters in a constant name are usually in uppercase. Unlike declaring variables,
the let keyword is not used to declare a constant.

*/

pub fn constants_types_run() {
    const USER_LIMIT: i32 = 100; // Declare a integer constant
    const PI: f32 = 3.14; //Declare a float constant

    println!("user limit is {}", USER_LIMIT); //Display value of the constant
    println!("pi value is {}", PI); //Display value of the constant
}
