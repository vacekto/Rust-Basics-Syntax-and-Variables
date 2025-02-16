use slug::slugify;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::process; // Import Write trait
fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

fn no_spaces(text: &str) -> String {
    text.replace(" ", "")
}

fn to_slug(text: &str) -> String {
    slugify(text)
}

fn no_transformation(text: &str) -> String {
    String::from(text)
}

fn reverse_chars(text: &str) -> String {
    text.chars().rev().collect()
}

fn reverse_words(text: &str) -> String {
    text.split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options: HashMap<&str, fn(&str) -> String> = HashMap::new();

    options.insert("lowercase", to_lowercase);
    options.insert("uppercase", to_uppercase);
    options.insert("slugify", to_slug);
    options.insert("no-spaces", no_spaces);
    options.insert("reverse-chars", reverse_chars);
    options.insert("reverse-words", reverse_words);
    options.insert("no-transformation", no_transformation);

    let keys: Vec<&str> = options.keys().cloned().collect();

    if args.len() != 2 {
        println!(
            "Invalid input, program requires exactly one argument. Possible options are: {:?}",
            keys
        );
        process::exit(1)
    }

    let selected = &args[1][..];

    if !keys.contains(&selected) {
        println!(
            "Invalid input \"{}\". Possible options are: {:#?}",
            selected, keys
        );
        process::exit(1)
    }

    let transform = options.get(selected).unwrap();
    let mut guess = String::new();

    print!("Enter text for transformation: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim_end();

    println!("Trasformed text: {}", transform(&guess));
}
