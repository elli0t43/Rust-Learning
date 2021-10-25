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
