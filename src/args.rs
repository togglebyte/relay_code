use std::env::args;

use crate::{
    actions::ActionKind,
    error::{Error, Result},
};

#[derive(Debug)]
pub enum Args {
    Action(ActionKind, String),
    New(String),
    Load(String),
    Help(Help),
    FeelingLucky,
}

#[derive(Debug)]
pub enum Help {
    General,
    Action,
}

impl Help {
    pub fn print(&self) {
        match self {
            Help::General => println!(
                "\
HELP!
-----
-h, --help        | Show this help
new <name>        | Create a new session
load <name>       | Load a session
action <action>   | Act upon a session
lucky            | Feeling lucky?"
            ),
            Help::Action => println!(
                "\
HELP for action!
----------------
action -h, --help | Show this help
action fight <target>
action love <target>
action neutral <target>
"
            ),
        }
    }
}

fn parse_action_kind<S: AsRef<str>>(action: S) -> Result<ActionKind> {
    match action.as_ref().to_lowercase().as_str() {
        "fight" => Ok(ActionKind::Fight),
        "love" => Ok(ActionKind::Love),
        "neutral" => Ok(ActionKind::Neutral),
        _ => Err(Error::InvalidActionType),
    }
}

impl Args {
    pub fn parse() -> Result<Args> {
        let mut args = args().skip(1);

        let next_arg = match args.next() {
            None => return Ok(Args::Help(Help::General)),
            Some(arg) => arg,
        };

        match next_arg.as_str() {
            "new" => {
                let name = args.next().ok_or(Error::InvalidArgs(""))?;
                Ok(Args::New(name))
            }
            "load" => {
                let name = args.next().ok_or(Error::InvalidArgs(""))?;
                Ok(Args::Load(name))
            }
            "action" => {
                if matches!(args.next().as_deref(), Some("-h" | "--help")) {
                    return Ok(Args::Help(Help::Action));
                }
                let action_arg = args.next().ok_or(Error::InvalidArgs("action "))?;
                let target_arg = args.next().ok_or(Error::InvalidArgs("action "))?;
                let action_arg = parse_action_kind(action_arg)?;
                log!("Action arg is {action_arg:?} where target_arg is {target_arg}");
                Ok(Args::Action(action_arg, target_arg))
            }
            "lucky" => Ok(Args::FeelingLucky),
            // "--help" | "-h" => Ok(Args::Help),
            _ => Ok(Args::Help(Help::General)),
        }
    }
}
