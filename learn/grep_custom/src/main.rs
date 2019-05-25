use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = grep_custom::InputArguments::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing parameters: {}", err);
            process::exit(1);
        });

    grep_custom::grep_in_file(config)
        .unwrap_or_else(|err| {
            println!("Problem reading file: {}", err);
            process::exit(1);
        });
}
