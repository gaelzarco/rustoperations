use std::io;

enum OperationSelection {
    Add(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    Subtract(i32, i32)
}

impl OperationSelection {
    fn perform_operation(&self) -> i32 {
        match self {
            OperationSelection::Add(x, y) => x + y,
            OperationSelection::Multiply(x, y) => x * y,
            OperationSelection::Divide(x, y) => x / y,
            OperationSelection::Subtract(x, y) => x - y
        }
    }
}

fn main() {
    println!("Please 'add', 'multiply', 'divide', or 'subract' a number");

    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut operation_selection: String = String::new();

    println!("Select an operation...");
    io::stdin()
    .read_line(&mut operation_selection)
    .expect("Failed to read line.");

    let operation: OperationSelection = match operation_selection.trim().to_lowercase().as_str() {
        "add" => {
            println!("Enter two numbers for addition:");
            io::stdin()
                .read_line(&mut x)
                .expect("Failed to read line.");

            io::stdin()
                .read_line(&mut y)
                .expect("Failed to read line.");

            let x: i32 = x.trim().parse().expect("Invalid number");
            let y: i32 = y.trim().parse().expect("Invalid number");

            OperationSelection::Add(x, y)
        },
        "multiply" => {
            println!("Enter two numbers for multiplication:");
            io::stdin()
                .read_line(&mut x)
                .expect("Failed to read line.");

            io::stdin()
                .read_line(&mut y)
                .expect("Failed to read line.");

            let x: i32 = x.trim().parse().expect("Invalid number");
            let y: i32 = y.trim().parse().expect("Invalid number");
            
            OperationSelection::Multiply(x, y)
        },
        "divide" => {
            println!("Enter two numbers for division:");
            io::stdin()
                .read_line(&mut x)
                .expect("Failed to read line.");

            io::stdin()
                .read_line(&mut y)
                .expect("Failed to read line.");

            let x: i32 = x.trim().parse().expect("Invalid number");
            let y: i32 = y.trim().parse().expect("Invalid number");

            OperationSelection::Divide(x, y)
        },
        "subtract" => {
            println!("Enter two numbers for subtraction:");
            io::stdin()
                .read_line(&mut x)
                .expect("Failed to read line.");

            io::stdin()
                .read_line(&mut y)
                .expect("Failed to read line.");

            let x: i32 = x.trim().parse().expect("Invalid number");
            let y: i32 = y.trim().parse().expect("Invalid number");

            OperationSelection::Subtract(x, y)
        },
        _ => {
            panic!("Invalid operation selection");
        }
    };

    let result = operation.perform_operation();
    println!("Result: {}", result);
}