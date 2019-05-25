use std::io;

fn main() {
    let mut size_of_list = String::new();

    println!("Enter length of list");
    io::stdin().read_line(&mut size_of_list)
        .expect("Input Error");

    let size_of_list: u32 = size_of_list.trim().parse()
        .expect("Enter a number as length of list.");

    let mut list: Vec<u32> = Vec::new();
    for _each in 0..size_of_list {
        println!("Enter next number:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Input Error");
        let number = input.trim().parse().expect("Not a number");

        list.push(number);
    }
    println!("The largest number is {}", find_largest(&list));
}

fn find_largest<T>(list: &[T]) -> T
    where T: Copy + PartialOrd
{
    let mut largest = list[0];

    for &each in list {
        if each > largest {
            largest = each
        }
    }
    largest
}
