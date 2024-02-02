use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Result;
use crate::serde::{Deserialize, Field, FieldReader, FieldType, Serialize, Serializer};
use crate::Entity;
use ActionKind::*;

fn start() -> i128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i128)
        .unwrap_or_else(|e| -(e.duration().as_millis() as i128))
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ActionKind {
    Fight,
    Love,
    Neutral,
    Spawn,
    Die,
    ElectroCute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub start: i128,
    pub entity: String,
    pub kind: ActionKind,
    pub target: Option<String>,
}

impl Action {
    pub fn interact(kind: ActionKind, entity: &Entity, target: &Entity) -> Result<Self> {
        let inst = Self {
            start: start(),
            kind,
            entity: entity.name.to_owned(),
            target: Some(target.name.to_string()),
        };
        Ok(inst)
    }

    pub fn spawn(entity: String) -> Self {
        Self {
            start: start(),
            entity,
            target: None,
            kind: Spawn,
        }
    }

    pub fn exec(&mut self, entity: &mut Entity) {
        let action: String = String::from("fight");
        log!("Action is : {action:?} and Entity is {0:?}", entity.name);
        match self.kind {
            ActionKind::Fight => {
                eprintln!("You're fighting {0}", entity.name);
            }
            _ => eprintln!("Typed unknown action you have"),
        }
    }
}

impl Serialize for Action {
    fn serialize(&self, buf: &mut Serializer) -> usize {
        let s = buf.unknown_size(FieldType::Action);
        let mut size = Field::I128(self.start).serialize(buf);
        size += Field::Str(self.entity.clone()).serialize(buf);
        size += Field::ActionKind(self.kind).serialize(buf);
        size += Field::Str(self.target.clone().unwrap_or_default()).serialize(buf);
        s(buf, size)
    }
}

impl Deserialize for Action {
    fn deserialize(reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        reader.ensure_type(FieldType::Action)?;
        let action = Self {
            start: reader.read_field()?,
            entity: reader.read_field()?,
            kind: reader.read_field()?,
            target: {
                let target: String = reader.read_field()?;
                (!target.is_empty()).then_some(target)
            },
        };

        Ok(action)
    }
}
