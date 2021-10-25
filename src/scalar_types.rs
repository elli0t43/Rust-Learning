/*
A scalar type represents a single value. For example, 10,3.14,'c'. Rust has four primary scalar types.

1. Integer
2. Floating-point
3. Booleans
4. Characters
*/

/*
-------------------------------------------------------------
# INTEGERS
An Integer data type is used to represent whole numbers.Integers can be further classified as Signed and Unsigned. Signed integers can store both negative and positive values. Unsigned integers can only store positive values.
A detailed description if integer types is given below −
|-------------------------------|
| Size	  | Signed |  Unsigned  |
| 8 bit	  |  i8	   |   u8       |
| 16 bit  | i16    |  u16       |
| 32 bit  | i32	   |  u32       |
| 64 bit  | i64	   |  u64       |
| 128 bit | i128   |  u128      |
| Arch	  | isize  |  usize     |
|-------------------------------|

The size of an integer can be arch. This means the size of the data type will be derived from the architecture of the machine. An integer the size of which is arch will be 32 bits on an x86 machine and 64 bits on an x64 machine.
An arch integer is primarily used when indexing some sort of collection.
*/

pub fn integer_types_run() {
    let basic_num = 10; // i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let marks_negetive: isize = -20;
    let count: usize = 30;

    println!("result value is {}", basic_num);
    println!("sum is {} and age is {}", sum, age);
    println!("mark is {} and count is {}", mark, count);
    println!("negetive mark is {}", marks_negetive);
}

// Integer Overflow
// An integer overflow occurs when the value assigned to an integer variable exceeds the Rust defined range for the data type.
pub fn integer_overflow_run() {
    //let age: u8 = 255;

    // 0 to 255 only allowed for u8
    //let weight: u8 = 256; //overflow value is 0
    //let height: u8 = 257; //overflow value is 1
    //let score: u8 = 258; //overflow value is 2

    //println!("age is {} ", age);
    //println!("weight is {}", weight);
    // println!("height is {}", height);
    // println!("score is {}", score);
    // the above code will return a error literal out of range for u8 for weight, height and score variables.
    // The overflow values after 255 will start from 0, 1, 2, etc.
}

/*
-------------------------------------------------------------
#FLOAT
Float data type in Rust can be classified as f32 and f64. The f32 type is a single-precision float, and f64 has double precision. The default type is f64.
DO KEEP IN MIND, TYPE-CASTING NOW ALLOWED IN RUST
*/

pub fn float_types_run() {
    let result = 10.00; //f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; //double precision

    println!("result value is {}", result);
    println!("interest is {}", interest);
    println!("cost is {}", cost);
}
// Number seperator
// For easy readability of large numbers, we can use a visual separator _ underscore to separate digits. That is 50,000 can be written as 50_000 . This is shown in the below example.
pub fn number_seperator_run() {
    let float_with_separator = 11_000.555_001;
    println!("float value {}", float_with_separator);
    let int_with_separator = 50_000;
    println!("int value {}", int_with_separator);
}
