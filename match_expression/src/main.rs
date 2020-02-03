enum Operation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

fn get_operation_type() -> Operation {
    Operation::Multiplication
}

fn main() {
    let x = 10;
    let y = 30;

    let result = match get_operation_type() {
        Operation::Multiplication => x * y,
        Operation::Division => x / y,
        Operation::Substraction => x - y,
        Operation::Addition => x + y,
    };
    println!("{}", result);
}
