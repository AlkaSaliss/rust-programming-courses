//! This modules contains math functions providing basic math operations like addition, subtraction, multiplication, etc.
//! # Examples
//! ```
//! use math_utils::math_functions::add;
//! let result = add(&2, &2);
//! println!("{} + {} = {}", 2, 2, result);
//! ```

/// This function adds two numbers.
/// # Examples
/// ```
/// use math_utils::math_functions::add;
/// let result = add(&2, &2);
/// println!("{} + {} = {}", 2, 2, result);
/// ```
pub fn add(x: &i32, y: &i32) -> i32 {
    x + y
}

/// This function subtracts two numbers.
/// # Examples
/// ```
/// use math_utils::math_functions::sub;
/// let result = sub(&2, &2);
/// println!("{} - {} = {}", 2, 2, result);
/// ```
pub fn sub(x: &i32, y: &i32) -> i32 {
    x - y
}

/// This function multiplies two numbers.
/// # Examples
/// ```
/// use math_utils::math_functions::mul;
/// let result = mul(&2, &2);
/// println!("{} * {} = {}", 2, 2, result);
/// ```
pub fn mul(x: &i32, y: &i32) -> i32 {
    x * y
}

/// This function divides two numbers.
/// # Examples
/// ```
/// use math_utils::math_functions::div;
/// let result = div(&2, &2);
/// println!("{} / {} = {}", 2, 2, result);
/// ```
pub fn div(x: &i32, y: &i32) -> i32 {
    x / y
}