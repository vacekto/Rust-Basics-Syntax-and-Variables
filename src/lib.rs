pub mod util;

use slug::slugify;
use std::{
    error::Error,
    io::{self, Write},
};
use util::structs::Csv;

fn get_line_input() -> Result<String, Box<dyn Error>> {
    let mut text_input = String::new();
    print!("Enter text for transformation: ");

    io::stdout().flush()?;
    io::stdin().read_line(&mut text_input)?;
    let text_input = String::from(text_input.trim());
    Ok(text_input)
}

fn get_csv_input() -> Result<Csv, Box<dyn Error>> {
    println!("To finish writing to CSV format, press \"ctrl + D\"");

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers()?;
    let headers_vec: Vec<String> = headers.iter().map(|x| x.to_string()).collect();

    let mut csv = Csv {
        headers: headers_vec,
        values: vec![],
    };

    for result in rdr.records() {
        let record: Vec<String> = result?.iter().map(|x| x.to_string()).collect();
        csv.values.push(record);
    }
    Ok(csv)
}

fn to_lowercase() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(text_input.to_lowercase())
}

fn to_uppercase() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(text_input.to_uppercase())
}

fn no_spaces() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(text_input.replace(" ", ""))
}

fn to_slug() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(slugify(text_input))
}

fn no_transformation() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(String::from(text_input))
}

fn reverse_chars() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(text_input.chars().rev().collect())
}

fn reverse_words() -> Result<String, Box<dyn Error>> {
    let text_input = get_line_input()?;
    Ok(text_input
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" "))
}

pub fn csv() -> Result<String, Box<dyn Error>> {
    let csv = get_csv_input()?;
    Ok(format!("{}", csv))
}

pub type Tranformation = fn() -> Result<String, Box<dyn Error>>;

pub const TRANFORMATION_OPTIONS: &[(&str, Tranformation)] = &[
    ("lowercase", to_lowercase),
    ("uppercase", to_uppercase),
    ("slugify", to_slug),
    ("no-spaces", no_spaces),
    ("reverse-chars", reverse_chars),
    ("reverse-words", reverse_words),
    ("no-transformation", no_transformation),
    ("csv", csv),
];
