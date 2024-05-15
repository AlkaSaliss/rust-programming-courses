use std::io;


fn main() {
    println!("Enter a greeting: ");
    let mut greeting = String::new();
    io::stdin().read_line(&mut greeting).expect("Failed to read line");
    
    match greeting.trim().to_lowercase().as_str() {
        "hello" => println!("Hi there!"),
        "goodbye" => println!("Goodbye!"),
        _ => println!("I don't know you"),
    }
}
