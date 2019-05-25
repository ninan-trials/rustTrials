use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct InputArguments {
    query: String,
    filename: String,
    case_sensitivity: bool,
}

impl InputArguments {
    pub fn new(args: &[String]) -> Result<InputArguments, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Parameters");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let mut case_sensitivity = false;
        if args.len() == 4 {
            let case_sensitivity_arg = args[3].clone();
            case_sensitivity = (case_sensitivity_arg.as_str() == "true") || (case_sensitivity_arg.as_str() == "1");
        }

        Ok(InputArguments { query, filename, case_sensitivity })
    }
}

pub fn grep_in_file(config: InputArguments) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;
    let mut matched_lines = Vec::new();
    if config.case_sensitivity {
        for line in search(&config.query, &content) {
            matched_lines.push(line);
            println!("{}", line);
        }
    } else {
        for line in search_insensitive(&config.query, &content) {
            matched_lines.push(line);
            println!("{}", line);
        }
    }
    if matched_lines.is_empty() {
        println!("No matches for {} in {}", config.query, config.filename);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            matches.push(line)
        }
    }
    matches
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    let mut matches: Vec<&str> = Vec::new();
    for line in content.lines() {
        let lowercase_line = line.to_lowercase();
        if lowercase_line.contains(&lowercase_query) {
            matches.push(line)
        }
    }
    matches
}

mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive_returns_line_when_matched() {
        let query = "ell";
        let content = "Hello, My Name is Leo";

        assert_eq!(
            vec![content],
            search(query, content)
        );
    }

    #[test]
    fn test_search_case_sensitive_returns_emoty_when_no_matches() {
        let query = "leo";
        let content = "Hello, My Name is Leo";

        assert!(
            search(query, content).is_empty()
        );
    }

    #[test]
    fn test_search_case_insensitive_returns_line_when_matched() {
        let query = "leo";
        let content = "Hello, My Name is Leo";

        assert_eq!(
            vec![content],
            search_insensitive(query, content)
        );
    }
}
