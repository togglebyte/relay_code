use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Result;
use crate::serde::{serialize, Deserialize, Field, FieldReader, Serialize};
use crate::Entity;

fn start() -> Result<u128> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    Ok(now)
}

#[derive(Debug, Clone)]
pub struct Action {
    start: u128,
    kind: String,
}

impl Action {
    pub fn new(kind: String) -> Result<Self> {
        let inst = Self {
            start: start()?,
            kind,
        };
        Ok(inst)
    }

    pub fn exec(&mut self, entity: &mut Entity) {
        match self.kind {
            _ => panic!(),
        }
    }
}

impl Serialize for Action {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        serialize(&mut bytes, Field::U128(self.start));
        serialize(&mut bytes, Field::Str(&self.kind));
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
        };

        Ok(action)
    }
}
