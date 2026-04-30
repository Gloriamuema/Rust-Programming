// Import the standard input/output library
// This allows the reading of user input from the console
use std::io;

fn main() {
    // Display calculator title and menu options to the user
    // `println!` is a macro that prints formatted text to the console, followed by a new line
    // The following lines output the title and menu options for the calculator
    // The emoji is used to make the output more visually appealing
    // The `println!` macro is used to print the title and menu options for the calculator
    println!("🧮 Simple Rust Calculator");
    println!("Choose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    // Create a mutable string to store the user's menu choice
    // `String::new()` creates a new empty string, and `mut` allows it to be modified
    // This variable will hold the user's input for the menu choice
    let mut choice = String::new();

    // Read user input from standard input and store it in `choice`
    // `expect` handles potential errors during input
    // The `io::stdin().read_line(&mut choice)` reads a line of input from the user and stores it in the `choice` variable
    // If there is an error while reading input, the program will panic and display the message "Failed to read input"
    // The `&mut choice` syntax indicates that we are passing a mutable reference to the `choice` variable, allowing the `read_line` method to modify its value
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    // Convert the input (String) into an unsigned integer (u32)
    // `trim()` removes whitespace/newline, `parse()` converts to number
    let choice: u32 = choice
        .trim()
        .parse()
        .expect("Please enter a number");

    // Prompt user for the first number
    println!("Enter first number:");

    // Read first number as a string
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input");

    // Convert first input into a floating-point number (f64)
    let num1: f64 = num1
        .trim()
        .parse()
        .expect("Enter a valid number");

    // Prompt user for the second number
    println!("Enter second number:");

    // Read second number as a string
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input");

    // Convert second input into a floating-point number (f64)
    let num2: f64 = num2
        .trim()
        .parse()
        .expect("Enter a valid number");

    // Perform the selected operation using a match expression
    let result = match choice {
        // Addition
        1 => num1 + num2,

        // Subtraction
        2 => num1 - num2,

        // Multiplication
        3 => num1 * num2,

        // Division with error handling for division by zero
        4 => {
            if num2 == 0.0 {
                println!("❌ Cannot divide by zero");
                return; // Exit program early
            }
            num1 / num2
        }

        // Handle invalid menu choices
        _ => {
            println!("❌ Invalid choice");
            return; // Exit program early
        }
    };

    // Display the final result to the user
    println!("✅ Result: {}", result);
}