use std::io;

fn main() {
    let mut first_num = -1;
    let mut second_num = 1;

    let mut limit = String::new();

    println!("Enter the limit:");
    io::stdin().read_line(&mut limit)
        .expect("Could not read input");

    let limit: u32 = limit.trim().parse().unwrap();

    for n in (0..limit) {
        let mut next_num = first_num + second_num;
        first_num = second_num;
        second_num = next_num;
    }

    println!("The nth generated number is {}", second_num);
}
