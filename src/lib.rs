use core::fmt;
use std::{env, error::Error};

pub mod payload;
pub mod fbs;
pub mod buffer;
pub mod consumer;
pub mod config;

#[derive(Debug)]
pub enum ArgsError {
    MissingArguments,
    MissingId
}

impl Error for ArgsError {}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArguments => write!(f, "no arguments provided"),
            Self::MissingId => write!(f, "missing argument 'id'")
        }
    }
}

pub fn get_bee_id_from_args() -> Result<u64, anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <query> <bee_id>", args[0]);
        return Err(ArgsError::MissingArguments.into());
    }

    let query= &args[1];
    if query != "id" {
        eprintln!("Please provide 'id' as an argument.");
        return Err(ArgsError::MissingId.into());
    }

    let arg_bee_id = &args[2];
    Ok(arg_bee_id.parse::<u64>()?)
}