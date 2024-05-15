fn main() {
    // let maybe_num = Some(5);
    // if let Some(num) = maybe_num {
    //     println!("This is a number: {}", num);
    // } else {
    //     println!("This is not a number");
    // }

    // let maybe_num: Option<Option<()>> = Some(None);
    // if let Some(num) = maybe_num {
    //     println!("This is a number: {:#?}", num);
    // } else {
    //     println!("This is not a number");
    // }

    let maybe_num = Some(Some(42));
    if let Some(num) = maybe_num {
        println!("This is a number: {:#?}", num);
    } else {
        println!("This is not a number");
    }
}
