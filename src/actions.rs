use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Result;
use crate::serde::{serialize, Deserialize, Field, FieldReader, Serialize};
use crate::Entity;

fn start() -> Result<u128> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    Ok(now)
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ActionKind {
    Fight,
    Love,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    start: u128,
    target: String,
    kind: ActionKind,
}

impl Action {
    pub fn new(kind: ActionKind, target: String) -> Result<Self> {
        let inst = Self {
            start: start()?,
            kind,
            target,
        };
        Ok(inst)
    }

    pub fn exec(&mut self, entity: &mut Entity) {
        let action: String = String::from("fight");
        eprintln!("Action is : {action:?} and Entity is {0:?}", entity.name);
        match self.kind {
            ActionKind::Fight => {
                eprintln!("You're fighting {0}", entity.name);
            }
            _ => eprintln!("Typed unknown action you have"),
        }
    }
}

impl Serialize for Action {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        serialize(&mut bytes, Field::U128(self.start));
        serialize(&mut bytes, Field::ActionKind(self.kind));
        serialize(&mut bytes, Field::Str(&self.target));
        eprintln!("BUFFER: {bytes:?}");
        bytes
    }
}

impl Deserialize for Action {
    fn deserialize(reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let action = Self {
            start: reader.read_field()?,
            kind: reader.read_field()?,
            target: reader.read_field()?,
        };

        Ok(action)
    }
}
