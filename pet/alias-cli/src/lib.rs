use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::process::Command;
use std::io::Write;

#[derive(Debug)]
pub struct InputArguments {
    alias_name: String,
    command: String,
    alias_file_path: String,
}

impl InputArguments {
    pub fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Parameters");
        }

        let alias_name = args[1].clone();
        let command = args[2].clone();
        let mut alias_file_path = String::from("./.alias");
        if args.len() == 4 {
            let alias_file_path_arg = args[3].clone();
            alias_file_path = String::from(alias_file_path_arg)
        }

        Ok(InputArguments { alias_name, command, alias_file_path })
    }
}

pub fn write_to_alias_file(config: InputArguments) -> Result<(), Box<dyn Error>> {
    let alias_name = config.alias_name;
    let command = config.command;
    let alias_line = String::from("\nalias ") + &alias_name + &String::from("=") + &command;
    let mut alias_file = OpenOptions::new()
        .append(true)
        .open(&config.alias_file_path)
        .unwrap();

    alias_file.write_all(alias_line.as_bytes());
//    writeln!(alias_file, alias_line)?;
//    fs::write(&config.alias_file_path, alias_line)?;
    source_alias_file(&config.alias_file_path)?;
    Ok(())
}

fn source_alias_file(alias_file: &String) -> Result<(), Box<dyn Error>> {
//    let source_command = String::from("source ") + alias_file;
//    println!("{}", source_command);
//
//    print!("{}", alias_file);
    Command::new("ls")
        .arg("-a")
        .spawn()
        .expect("Failed to list");

    Command::new("bash")
        .arg("-c")
        .arg(format!("source {}", alias_file))
        .spawn()
        .expect("Failed to source alias file");

    Command::new("bash")
        .arg("-c")
        .arg("exec")
        .arg("zsh")
        .spawn()
        .expect("Failed to reload shell");


    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_write_alias_writes_to_alias_file() {
        let alias_name = String::from("l");
        let command = String::from("ls");
        let alias_file_path = String::from("./test_alias");
        let test_config = InputArguments { alias_name, command, alias_file_path };

        let alias_line = String::from("alias ") + &alias_name + &String::from("=") + &command;

        write_to_alias_file(test_config);
        let alias_file_content = fs::read_to_string(&test_config.alias_file_path)?;

        assert!(alias_file_content.contains(alias_line));
    }
}
