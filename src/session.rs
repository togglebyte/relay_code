use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use crate::actions::Action;
use crate::error::{Error, Result};
use crate::serde::{serialize, Deserialize, Field, FieldReader, Serialize};
use crate::Entity;

const FILENAME: &str = "entity.lol";

#[derive(Debug, PartialEq)]
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
            eprintln!("No entity found");
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
        eprintln!("Deserializing");
        let entity = reader.read_field()?;
        let action = reader.read_field()?;
        eprintln!("Action: {:?}", action);

        let entity = Self { action, entity };

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

#[cfg(test)]
mod tests {
    use crate::{
        actions::Action,
        serde::{Deserialize, FieldReader, Serialize},
        Entity,
    };

    use super::Session;

    fn deserialize<T: Deserialize>(bytes: &[u8]) -> crate::error::Result<T> {
        let mut reader = FieldReader::new(bytes);
        T::deserialize(&mut reader)
    }

    #[test]
    fn session_round_trip() {
        let session = Session {
            action: Action::new("Fight".to_string()).unwrap(),
            entity: crate::Entity {
                name: "florp".to_string(),
                field_b: 69,
                field_c: true,
            },
        };

        let serialized = session.serialize();
        let actual = deserialize::<Session>(&serialized).unwrap();

        assert_eq!(actual, session);
    }

    #[test]
    fn action_round_trip() {
        let expected = Action::new("Fight".to_string()).unwrap();
        let serialized = expected.serialize();
        let actual = deserialize::<Action>(&serialized).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn entity_round_trip() {
        let expected = Entity {
            name: "florp".to_string(),
            field_b: 69,
            field_c: true,
        };
        let serialized = expected.serialize();
        eprintln!("BYTES: {serialized:?}");
        let actual = deserialize::<Entity>(&serialized).unwrap();

        assert_eq!(actual, expected);
    }
}
