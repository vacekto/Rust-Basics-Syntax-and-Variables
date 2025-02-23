use std::{error::Error, fmt};

use crate::TRANFORMATION_OPTIONS;

pub enum CliArgError {
    NoArgProvided,
    WrongArg,
}

impl fmt::Display for CliArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let options: Vec<&'static str> = TRANFORMATION_OPTIONS.iter().map(|x| x.0).collect();
        match self {
            CliArgError::NoArgProvided => write!(f, "Invalid input, program needs an argument to specify string transformation. Possible options are: {} ", options.join(", ")),
            CliArgError::WrongArg => write!(f, "Invalid argument provided, possible options are: {}", options.join(", ")),
        }
    }
}

impl fmt::Debug for CliArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CliArgError {}
