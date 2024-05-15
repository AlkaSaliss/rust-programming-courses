use math_utils::{calculate, MathOperator};


fn main() {
    // Add two numbers
    calculate(MathOperator::Add, 2, 2);

    // Subtract two numbers
    calculate(MathOperator::Sub, 2, 2);

    // Multiply two numbers
    calculate(MathOperator::Mul, 2, 2);

    // Divide two numbers
    calculate(MathOperator::Div, 2, 2);
}
