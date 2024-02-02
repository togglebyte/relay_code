//use std::io::Cursor;

use args::Args;
use error::Result;
use serde::{Deserialize, Field, FieldReader, FieldType, Serialize, Serializer};
use session::Session;

#[macro_use]
mod log;
mod actions;
mod args;
mod error;
mod serde;
mod session;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub name: String,
    health: u8,
    field_c: bool,
}

impl Entity {
    pub fn new(name: String) -> Self {
        Self {
            name,
            health: 5,
            field_c: false,
        }
    }
}

impl Serialize for Entity {
    fn serialize(&self, buf: &mut Serializer) -> usize {
        let s = buf.unknown_size(FieldType::Entity);
        let mut size = Field::Str(self.name.clone()).serialize(buf);
        size += Field::Byte(self.health).serialize(buf);
        size += Field::Bool(self.field_c).serialize(buf);
        s(buf, size)
    }
}

impl Deserialize for Entity {
    fn deserialize(reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        reader.ensure_type(FieldType::Entity)?;
        let entity = Self {
            name: reader.read_field()?,
            health: reader.read_field()?,
            field_c: reader.read_field()?,
        };

        Ok(entity)
    }
}

fn main() -> Result<()> {
    log::set_log();
    let args = Args::parse()?;

    //let session = Session::load().unwrap();
    match args {
        Args::Help(help) => help.print(),
        Args::Action(kind, target) => {
            log!("args are {kind:?} and {target}");
            let session = Session::load(&target)?;
            eprintln!("Session comprises of: {session:?}");
        }
        Args::New(name) => {
            let entity = Entity::new(name);
            let session = Session::new(entity)?;
            session.save()?;
            eprintln!("session saved");
        }
        Args::Load(name) => {
            log!("name is {name:?}");
            let session = Session::load(&name)?;
            eprintln!("{session}");
        }
    }

    Ok(())
}
