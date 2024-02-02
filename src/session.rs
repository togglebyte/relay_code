use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

use crate::actions::Action;
use crate::error::{Error, Result};
use crate::serde::{serialize, Deserialize, FieldReader, FieldType, Serialize, Serializer};
use crate::Entity;

const DIRNAME: &str = "sessions";

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Session {
    party: Vec<Entity>,
    opponents: Vec<Entity>,
    actions: Vec<Action>,
}

fn table_row<T>(
    f: &mut Formatter<'_>,
    col_width: usize,
    values: impl IntoIterator<Item = T>,
    mut extr: impl FnMut(T) -> String,
) -> fmt::Result {
    for value in values {
        let value = &extr(value)[..col_width];
        write!(f, "{value:col_width$}  ")?;
    }
    writeln!(f)
}

impl Display for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Oponents:\n=========")?;
        table_row(f, 15, &self.opponents, |o| o.name.to_string())?;
        table_row(f, 15, &self.opponents, |o| {
            format!("{:♥<1$}", "", o.health as usize)
        })?;

        writeln!(f, "\n\n")?;

        writeln!(f, "Party:\n=========")?;
        table_row(f, 15, &self.opponents, |o| o.name.to_string())?;
        table_row(f, 15, &self.opponents, |o| {
            format!("{:♥<1$}", "", o.health as usize)
        })?;
        Ok(())
    }
}

impl Session {
    pub fn new(entity: Entity) -> Result<Self> {
        let inst = Self {
            actions: vec![Action::spawn(entity.name.to_string())],
            party: vec![entity],
            ..Default::default()
        };
        Ok(inst)
    }
}

fn session_file(name: &str) -> Result<File, std::io::Error> {
    fs::create_dir_all(DIRNAME)?;
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(format!("{DIRNAME}/{name}.the_most_powerful.lol"))?;
    Ok(file)
}

impl Session {
    pub fn load(name: &str) -> Result<Self> {
        let mut file = match session_file(name) {
            Err(e) if matches!(e.kind(), io::ErrorKind::NotFound) => return Err(Error::NoSession),
            e => e?,
        };
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        if bytes.is_empty() {
            log!("No session found");
            return Err(Error::NoSession);
        }
        let mut reader = FieldReader::new(bytes.as_slice());
        Self::deserialize(&mut reader)
    }

    pub fn save(&self) -> Result<()> {
        let mut file = session_file(&self.party[0].name)?;
        let bytes = serialize(self);
        file.write_all(&bytes)?;
        Ok(())
    }
}

impl Deserialize for Session {
    fn deserialize(reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        log!("Deserializing");
        reader.ensure_type(FieldType::Session)?;
        let session = Self {
            party: reader.read_field()?,
            opponents: reader.read_field()?,
            actions: reader.read_field()?,
        };
        log!("{session:?}");

        Ok(session)
    }
}

impl Serialize for Session {
    fn serialize(&self, buf: &mut Serializer) -> usize {
        let s = buf.unknown_size(FieldType::Session);
        let mut size = self.party.serialize(buf);
        size += self.opponents.serialize(buf);
        size += self.actions.serialize(buf);
        s(buf, size)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{Action, ActionKind},
        serde::{serialize, Deserialize, Field, FieldReader},
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
            actions: vec![
                Action::spawn("Toerktumlare".to_string()),
                Action {
                    kind: ActionKind::Fight,
                    entity: "Gilgamesh".to_string(),
                    start: 1234,
                    target: Some("Tommy".to_string()),
                },
            ],
            party: vec![crate::Entity {
                name: "florp".to_string(),
                field_c: true,
                health: 69,
            }],
            ..Default::default()
        };

        let serialized = serialize(&session);
        let actual = deserialize::<Session>(&serialized).unwrap();

        assert_eq!(actual, session);
    }

    #[test]
    fn action_round_trip() {
        let expected = Action::spawn("".to_string());
        let serialized = dbg!(serialize(&expected));
        let actual = deserialize::<Action>(&serialized).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn entity_round_trip() {
        let expected = Entity {
            name: "florp".to_string(),
            health: 69,
            field_c: true,
        };
        let serialized = serialize(&expected);
        eprintln!("BYTES: {serialized:?}");
        let actual = deserialize::<Entity>(&serialized).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn bool_round_trip() {
        let bool = Field::Bool(true);
        let bytes = serialize(&bool);
        assert_eq!(bytes, [4, 0, 1, 1]);
        assert_eq!(bool, deserialize(&bytes).unwrap());
    }
}
