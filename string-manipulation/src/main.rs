

fn main() {
    let mystring = String::from("Hello, world!");
    let mystring2 = "Hello, world!".to_string();

    println!("{} {}", mystring, mystring2);
    let str_are_equal = mystring == mystring2;
    println!("{}", str_are_equal);

    let myslice = &mystring[0..=5];
    println!("{}", myslice);

    // get vowels
    for c in mystring.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("{}", c),
            _ => continue
        }
    }

    //split string
    let splitted: Vec<&str> = mystring.split(",").collect();

    println!("{:#?}", splitted);

    // reverse string
    let reversed = mystring.chars().rev().collect::<String>();
    println!("{}", reversed);

    // replace
    let replaced = mystring.replace("world", "Rust");
    println!("{}", replaced);
}
