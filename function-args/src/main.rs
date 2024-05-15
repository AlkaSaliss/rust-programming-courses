use std::io;


fn sum(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

fn average(nums: &Vec<i32>) -> f32 {
    let sum = sum(&nums);
    sum as f32 / nums.len() as f32
}


fn main() {
    // let nums = [1, 2, 3, 4, 5];
    // let result = sum(&nums);
    // println!("{}", result);
    let mut input = String::new();
    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..3 {
        println!("Enter a number: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Failed to parse number");
        nums.push(num);
        input.clear();
    }

    let result = sum(&nums);
    let avg = average(&nums);
    println!("Sum: {}", result);
    println!("Average: {}", avg);
}
