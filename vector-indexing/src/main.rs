
fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_vector(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in vec {
        sum += value;
    }
    sum
}

fn main() {
    let mut vec  = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value: i32 = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    // Retrieve the last value using pattern matching
    match vec.last() {
        Some(last_value) => println!("The last value in the vector is: {}", last_value),
        None => println!("The vector is empty!"),
    }

    let sum = sum_vector(&vec);
    println!("The sum of the vector is: {}", sum);

    let some_val = vec.get(5).unwrap_or_else( || &0 );
    println!("The value at index 5 is: {}", some_val);

    // append to the vector
    let mut new_vec = vec![6, 7, 8];
    println!("Before append  The new vector to add is: {:?}", new_vec);
    vec.append(&mut new_vec);

    println!("The new vector is: {:?}", vec);
    println!("The new vector to add is: {:?}", new_vec);

    // insert into the vector
    vec.insert(2, 10);
    println!("The new vector is: {:?}", vec);
}
