#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    age: u8,
}

impl User {

    fn new(username: String, email: String,active: bool, age: u8) -> User {
        User {
            email,
            username,
            active,
            age
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.email, self.username)
    }

    fn deactivate_user(&mut self) {
        self.active = !self.active
    }
}

fn main() {

    let mut user1 = User::new(String::from("someusername123"),String::from("h7h7h@example.com"),true,20);   
    println!(
        "{:#?}",
        user1
    );

    user1.deactivate_user();
    println!(
        "{:#?}",
        user1
    );
    println!("Hello, {}!", user1.get_full_name());
}
