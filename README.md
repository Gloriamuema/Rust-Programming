# Rust-Programming
This repository represents my hands-on journey into Rust programming as part of my capstone project at Moringa School under the AI Access for Software Developers course. It reflects my growth in writing safe, efficient, and modern code while exploring real-world problem solving using Artificial intelligence

# What is Rust?

Rust is a modern programming language designed for:
Speed (like C/C++)
Safety (prevents bugs like crashes and memory leaks)
Concurrency (great for handling multiple tasks efficiently)
It was originally created by Mozilla.

# Why Learn Rust?
Rust is worth learning because:
1. Memory Safety (No crashes)
Rust prevents common bugs automatically (no null pointer errors, no memory leaks).
2. High Performance
It’s used where speed matters—like:
Operating systems
Game engines
Backend systems
3. Industry Adoption
The following companies are using Rust in real systems:
i. Microsoft
ii. Amazon
iii. Google
.

# Rust is used in:
1. Backend development (APIs, servers)
2. Systems programming (OS, drivers)
3. WebAssembly (WASM) for web apps
4. Embedded systems
5. CLI tools (command-line apps)

# Common Rust Libraries (Crates)
In Rust, libraries are called crates.
Popular ones include:
1. serde → for JSON/data handling
2. tokio → async programming
3. reqwest → HTTP requests
4. clap → CLI apps
5. actix-web → web backend framework

# Variables in Rust
Rust is strict but simple.

1. Basic variable
let x = 5;
2. Mutable variable (can change)
let mut x = 10;
x = 20;
3. Constant
const MAX: i32 = 100;

# STEP-BY-STEP: My First Rust Project
1. Install Rust
Go to: https://rustup.rs
Download and install.
Then verify:
cargo --version 

1. Create a Project
Run this in terminal:
cargo new hello_rust 
This uses Cargo a Rust tool for managing projects

3. Open the Folder
cd hello_rust
4. Project Structure

You will see:

hello_rust/
 ├── Cargo.toml   ← project config
 └── src/
     └── main.rs  ← main code
5. Write the First Code

Open src/main.rs and replace with:

fn main() {
    println!("Hello Gloria , Welcome to rust world!");
}

6. Run the Program
in the terminal still at hello_rust folder
type:
cargo run

You should see:
Hello Gloria , Welcome to rust world!

# How to Run because I already have the project on Github

1. Install Rust: https://rustup.rs

2. Clone this repository:

git clone https://github.com/Gloriamuema/Rust-Programming.git

3. Navigate into the project:
cd Rust-Programming
cd hello_rust

4. Run the project:
cargo run