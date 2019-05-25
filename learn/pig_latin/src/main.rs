use std::io;

fn main() {
    let mut input_string = String::new();
    println!("Enter String:");
    io::stdin().read_line(&mut input_string)
        .expect("Input Error");

    let mut result_string = String::from("");
    for word in input_string.split_whitespace() {
        result_string += &convert_to_piglatin(word);
    }
    println!("Result: {}", result_string);
}

fn convert_to_piglatin(word: &str) -> String {
    let len = word.len();
    let first_letter = &word[0..1];
    let mut converted_word = String::from("");
    if is_vowel(&first_letter) {
        converted_word = String::from(&word[0..len]);
        converted_word += "hay ";
        converted_word
    } else {
        converted_word = String::from(&word[1..len]);
        let first_letter = String::from(first_letter);
        converted_word = converted_word + &first_letter + "ay ";
        converted_word
    }
}

fn is_vowel(letter: &str) -> bool {
    letter == "a" || letter == "e" || letter == "i" || letter == "o" || letter == "u"
}
