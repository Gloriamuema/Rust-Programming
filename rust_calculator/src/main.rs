use std::io;

fn main() {
    println!("🧮 Simple Rust Calculator");
    println!("Choose an operation:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Enter a valid number");

    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Enter a valid number");

    let result = match choice {
        1 => num1 + num2,
        2 => num1 - num2,
        3 => num1 * num2,
        4 => {
            if num2 == 0.0 {
                println!("❌ Cannot divide by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("❌ Invalid choice");
            return;
        }
    };

    println!("✅ Result: {}", result);
}