
fn main() {
    let proceed = true;
    if proceed {
        println!("Hello, world!");
    } else {
        println!("Goodbye, world!");
    }

    let height = 5;
    if height < 4 {
        println!("short");
    } else if height < 8 {
        println!("medium");
    } else {
        println!("tall");
    }
}
