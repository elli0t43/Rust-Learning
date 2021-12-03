# Rust-Learning
Beginner Rust Learning, I'll update it alongside my rust learning.

### What is Rust?
Rust is a low-level statically-typed multi-paradigm programming language that’s focused on safety and performance.

Rust solves problems that C/C++ has been struggling with for a long time, such as memory errors and building concurrent programs.

It has three main benefits:

1. better memory safety due to the compiler;
2. easier concurrency due to the data ownership model that prevents data races;
3. zero-cost abstractions.

### Installation
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or 
```bash
$ curl https://sh.rustup.rs -sSf | sh
```

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
If you want to know more about cargo commands, refer [here.](https://doc.rust-lang.org/cargo/commands/cargo.html)

>Warning: Rust dones't use Pascal Case ( PascalCase ) or Camel Case ( camelCase ), Rust uses Snake Case ( snake_case )  

>Pro Tip: To format your rust code use command `rustfmt`
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
