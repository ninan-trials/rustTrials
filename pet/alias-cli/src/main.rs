use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = alias_cli::InputArguments::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing parameters: {}", err);
            process::exit(1);
        });

    alias_cli::write_to_alias_file(config)
        .unwrap_or_else(|err| {
            println!("Problem writing alias to file: {}", err);
            process::exit(1);
        });
}
