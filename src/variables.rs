/*
# NAMEING A VARIABLE
Rules for Naming a Variable
    1. The name of a variable can be composed of letters, digits, and the underscore character.

    2. It must begin with either a letter or an underscore.

    3. Upper and lowercase letters are distinct because Rust is case-sensitive.
*/

pub fn var_run() {
    // with let keyword we can define
    let my_name = "TutorialsPoint"; // string type
    let random_float = 4.5; // float type
    let random_boolean = true; // boolean type
    let random_char = '♥'; //unicode character type

    println!("My name is: {}", my_name);
    println!("Random Float number: {}", random_float);
    println!("Random boolean: {}", random_boolean);
    println!("Random Character: {}", random_char);
}
