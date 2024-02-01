use std::env::args;

use crate::error::{Error, Result};

#[derive(Debug)]
pub enum Args {
    Action(String),
    New(String),
    Load(String),
    Help,
}

impl Args {
    pub fn parse() -> Result<Args> {
        let mut args = args().skip(1);

        let next_arg = match args.next() {
            None => return Ok(Args::Help),
            Some(arg) => arg,
        };

        match next_arg.as_str() {
            "new" => {
                let name = args.next().ok_or(Error::InvalidArgs)?;
                Ok(Args::New(name))
            }
            "load" => {
                let name = args.next().ok_or(Error::InvalidArgs)?;
                Ok(Args::Load(name))
            }
            "action" => {
                let action_arg = args.next().ok_or(Error::InvalidArgs)?;
                eprintln!("Action arg is {action_arg:?}");
                Ok(Args::Action(action_arg))
            }
            "--help" | "-h" => Ok(Args::Help),
            _ => Ok(Args::Help),
        }
    }
}
