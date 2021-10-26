# Rust-Learning
Beginner Rust Learning, I'll update it alongside my rust learning.

### Basic Rust Commands
1. To create a project inside the existing directory, 
```bash
cargo init
```
2. To create a new project with out any existing directory, 
```bash
cargo new NameOfTheProject
```
3. To compile and it without rustc utility,
```bash
carge run
```
4. To compile it without running, 
```bash
cargo build
```
5. To build for production, 
```bash
cargo build --release
```
6. To check your code and make sure it compiles but doesn’t produce an executable,
```bash
cargo check
```

<details>
<summary><b>What is TOML ?</b></summary>

This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

The first line, [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. 
</details>

<details>
<summary><b>Constants v/s Variables</b></summary>

1. Constants are declared using the const keyword while variables are declared using the let keyword.

2. A variable declaration can optionally have a data type whereas a constant declaration must specify the data type. This means const USER_LIMIT=100 will result in an error.

3. A variable declared using the let keyword is by default immutable. However, you have an option to mutate it using the mut keyword. Constants are immutable.

4. Constants can be set only to a constant expression and not to the result of a function call or any other value that will be computed at runtime.

5. Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.

</details>