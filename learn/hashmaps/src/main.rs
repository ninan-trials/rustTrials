use std::io;
use std::collections::HashMap;

fn main() {
    let mut number_of_employees = String::new();

    println!("Enter number of employees");
    io::stdin().read_line(&mut number_of_employees)
        .expect("Input Error");

    let number_of_employees: i32 = number_of_employees.trim().parse().unwrap();
    let employee_department_map = create_map_of_employees(&number_of_employees);

    println!("{:?}", employee_department_map);
}

fn create_map_of_employees(number_of_employees: &i32) -> HashMap<String, Vec<String>> {
    let mut employee_department_map: HashMap<String, Vec<String>> = HashMap::new();

    for _ in 0..*number_of_employees {
        let mut employee_name = String::new();
        let mut department_name = String::new();
        println!("Enter Employee name:");
        io::stdin().read_line(&mut employee_name)
            .expect("Input Error");

        println!("Enter Department name:");
        io::stdin().read_line(&mut department_name)
            .expect("Input Error");
        let new_or_existing_vector =
            employee_department_map
                .entry(String::from(department_name.trim()))
                .or_insert_with(Vec::new);

        new_or_existing_vector.push(String::from(employee_name.trim()));
        new_or_existing_vector.sort();
    }
    employee_department_map
}
