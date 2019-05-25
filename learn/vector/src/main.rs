use std::io;
use std::collections::HashMap;

fn main() {
    let mut count_input = String::new();
    println!("How many numbers ?");

    io::stdin().read_line(&mut count_input)
        .expect("Input Error");

    let count: i32 = count_input.trim().parse().unwrap();

    let mut vector = Vec::new();
    vector = parse_input_to_vector(count, vector);

    let mean: i32 = compute_mean(count, &vector);
    let mode: i32 = find_mode(&vector);

    println!("Vector is: {:?} and its Mean is {}", vector, mean);
    println!("Its Mode is {}", mode);
}

fn parse_input_to_vector(count: i32, mut vector: Vec<i32>) -> Vec<i32> {
    for _ in 0..count {
        println!("Enter the number:");
        let mut input_number = String::new();

        io::stdin().read_line(&mut input_number)
            .expect("Input Error");

        let number: i32 = input_number.trim().parse().unwrap();
        vector.push(number);
    };
    vector
}

fn compute_mean(count: i32, vector: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for each in vector.iter() {
        sum += each;
    }
    println!("Sum is {}", sum);
    sum / count
}

fn find_mode(vector: &Vec<i32>) -> i32 {
    let mut mode_map = HashMap::new();
    let mut mode: i32 = 0;
    for each in vector.iter() {
        let count_of_occurence = mode_map.entry(each).or_insert(0);
        *count_of_occurence += 1;
        if *count_of_occurence > mode {
            mode = *each;
        }
    }
    println!("Hashmap is {:?}", mode_map);
    mode
}