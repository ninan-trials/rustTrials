use std::io;

fn main() {
    let mut input_temp = String::new();
    let mut input_measurement_unit = String::new();

    println!("Enter Temperature");
    io::stdin().read_line(&mut input_temp)
        .expect("Error getting input");

    let input_temp: f64 = match input_temp.trim().parse() {
        Ok(t) => t,
        Err(_) => {
            panic!("{} is not a number", input_temp.trim());
        }
    };

    println!("Enter Temperature Measurement unit");
    io::stdin().read_line(&mut input_measurement_unit)
        .expect("Error getting input");

    let output_temp;
    match input_measurement_unit.as_str().trim() {
        "celsius" => { output_temp = convert_to_fahrenheit(input_temp); }
        "fahrenheit" => { output_temp = convert_to_celsius(input_temp); }
        _default => panic!("Enter either 'celsius' or 'fahrenheit'")
    }

    println!("Converted {} to {}", input_measurement_unit.trim(), output_temp.trim());
}

fn convert_to_fahrenheit(celsius: f64) -> String {
    let output_temp = (celsius * 9.0 / 5.0) + 32.0;
    let result = format!("{} °F", output_temp);
    result
}

fn convert_to_celsius(fahrenheit: f64) -> String {
    let output_temp = (fahrenheit - 32.0) * (5.0 / 9.0);
    let result = format!("{} °C", output_temp);
    result
}

