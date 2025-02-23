use rust_basics::util::errors::CliArgError;
use rust_basics::{Tranformation, TRANFORMATION_OPTIONS};
use std::error::Error;
use std::{env, io};

fn get_transformation(selected_option: &str) -> Result<Tranformation, CliArgError> {
    TRANFORMATION_OPTIONS
        .iter()
        .find(|tranformation| tranformation.0 == selected_option)
        .map(|tranformation| tranformation.1)
        .ok_or(CliArgError::WrongArg)
}

fn get_cli_argument() -> Result<String, CliArgError> {
    let args: Vec<String> = env::args().collect();
    args.get(1).cloned().ok_or(CliArgError::NoArgProvided)
}

fn main() -> Result<(), Box<dyn Error>> {
    let selected = get_cli_argument()?;
    let tranformation = get_transformation(&selected)?;
    let tranformed = tranformation()?;
    println!("Trasformed text: {}", tranformed);
    Ok(())
}
