//! This library provide utility functions to simulate a calculator

pub mod math_functions;

/// Provide list of operators
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div
}

/// This function accepts two numbers and an operator and prints the result
/// # Examples
/// ```
/// use math_utils::calculate;
/// use math_utils::MathOperator;
/// calculate(MathOperator::Add, 2, 2);
/// ```
pub fn calculate(math_operator: MathOperator, x: i32, y: i32) {
    match math_operator {
        MathOperator::Add => println!("{} + {} = {}", x, y, x + y),
        MathOperator::Sub => println!("{} - {} = {}", x, y, x - y),
        MathOperator::Mul => println!("{} * {} = {}", x, y, x * y),
        MathOperator::Div => println!("{} / {} = {}", x, y, x / y),
    }
}
