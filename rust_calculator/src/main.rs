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
    // The `choice.trim().parse()` converts the user's input from a string to an unsigned 32-bit integer (u32)
    // The `expect` method is used to handle any potential errors that may occur during the
    // conversion process. If the user enters an invalid number, the program will panic and display the message "Please enter a number"
    // The `let choice: u32` syntax declares a new variable named `choice` of type `u32` and assigns it the value obtained from parsing the user's input
    // The `trim()` method is used to remove any leading or trailing whitespace from the input string, and the `parse()` method attempts to convert the trimmed string into a `u32` integer. If the conversion fails (e.g., if the user enters a non-numeric value), the program will panic and display the specified error message.
    // The `choice` variable now holds the user's menu selection as a number, which will be used later to determine which operation to perform
    let choice: u32 = choice
        .trim()
        .parse()
        .expect("Please enter a number");

    // Prompt user for the first number
    // The following line outputs a prompt asking the user to enter the first number for the calculation
    println!("Enter first number:");

    // Read first number as a string
    // Create a mutable string to store the user's first number input
    // The `String::new()` creates a new empty string, and `mut` allows it to be modified
    // This variable will hold the user's input for the first number
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input");

    // Convert first input into a floating-point number (f64)
    // The `num1.trim().parse()` converts the user's input from a string to a floating-point number (f64)
    // The `expect` method is used to handle any potential errors that may occur during the conversion process. If the user enters an invalid number, the program will panic and display the message "Enter a valid number"
    // The `let num1: f64` syntax declares a new variable named `num1` of type `f64` and assigns it the value obtained from parsing the user's input
    // The `trim()` method is used to remove any leading or trailing whitespace from the input string, and the `parse()` method attempts to convert the trimmed string into a `
    
    let num1: f64 = num1
        .trim()
        .parse()
        .expect("Enter a valid number");

    // Prompt user for the second number
    // The following line outputs a prompt asking the user to enter the second number for the calculation
    // The `println!` macro is used to print a message asking the user to enter the second number for the calculation
    // The emoji is used to make the output more visually appealing
    println!("Enter second number:");

    // Read second number as a string
    // Create a mutable string to store the user's second number input
    // The `String::new()` creates a new empty string, and `mut` allows it to be modified
    // This variable will hold the user's input for the second number   
    // The `io::stdin().read_line(&mut num2)` reads a line of input from the user and stores it in the `num2` variable
    //  If there is an error while reading input, the program will panic and display the message "Failed to read input"
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input");

    // Convert second input into a floating-point number (f64)
    // The `num2.trim().parse()` converts the user's input from a string to a floating-point number (f64)
    // The `expect` method is used to handle any potential errors that may occur during the
    // conversion process. If the user enters an invalid number, the program will panic and display the message "Enter a valid number"
    // The `let num2: f64` syntax declares a new variable named `num2` of type `f64` and assigns it the value obtained from parsing the user's input    
    let num2: f64 = num2
        .trim()
        .parse()
        .expect("Enter a valid number");

    // Perform the selected operation using a match expression
    // The `match` expression is used to determine which operation to perform based on the user's menu choice
    // Each arm of the `match` corresponds to a different operation (addition, subtraction,
    // multiplication, division) and calculates the result accordingly
    // The division arm includes error handling to prevent division by zero, which would cause a runtime error
    // The `_` arm handles any invalid menu choices by printing an error message and exiting the program early
    // The result of the selected operation is stored in the `result` variable, which is then displayed to the user
    //
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
    // The `println!` macro is used to print the result of the calculation to the console
    // The emoji is used to make the output more visually appealing
    println!("✅ Result: {}", result);
}