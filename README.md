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
### What is TOML ?
This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

The first line, [package], is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. 