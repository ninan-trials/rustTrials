use std::env;
use std::process;
use gocd_pipeline_automator as gocd_api_interface;
use gocd_pipeline_automator::InputArguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input = InputArguments::new(&args);

    let input = input
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

    gocd_api_interface::create_pipeline_from_file(input)
        .unwrap_or_else(|err| {
            println!("Failed creating a pipeline: {}", err);
            process::exit(1);
        });
}

