use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read, Write};
use std::path::Path;

use crate::actions::Action;
use crate::error::{Error, Result};
use crate::serde::{serialize, Deserialize, Field, FieldReader, Serialize};
use crate::Entity;

const FILENAME: &str = "entity.lol";

#[derive(Debug)]
pub struct Session {
    action: Action,
    entity: Entity,
}

impl Session {
    pub fn new(entity: Entity) -> Result<Self> {
        let inst = Self {
            entity,
            action: Action::new("no action".into())?,
        };
        Ok(inst)
    }
}

fn entity_file() -> Result<File> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(FILENAME)?;
    Ok(file)
}

impl Session {
    pub fn load() -> Result<Self> {
        let mut file = entity_file()?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        if bytes.is_empty() {
            println!("No entity found");
            return Err(Error::NoEntity);
        }
        let mut reader = FieldReader::new(bytes.as_slice());
        Self::deserialize(&mut reader)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = entity_file()?;
        let bytes = self.serialize();
        file.write_all(&bytes)?;
        Ok(())
    }
}

impl Deserialize for Session {
    fn deserialize(reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        println!("Deserializing");
        let action = reader.read_field()?;
        println!("Action: {:?}", action);
        let entity = reader.read_field()?;

        let entity = Self {
            action,
            entity,
        };

        Ok(entity)
    }
}

impl Serialize for Session {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        serialize(&mut bytes, Field::Entity(self.entity.clone()));
        serialize(&mut bytes, Field::Action(self.action.clone()));
        bytes
    }
}
