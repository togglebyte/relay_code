use crate::actions::{Action, ActionKind};
use crate::error::{Error, Result};
use crate::session::Session;
use crate::strings::Boolean;
use crate::Entity;

pub fn serialize(value: &impl Serialize) -> Vec<u8> {
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer);
    serializer.0
}

#[derive(Default)]
pub struct Serializer(Vec<u8>);

impl Serializer {
    pub fn unknown_size(&mut self, field_type: FieldType) -> impl Fn(&mut Self, usize) -> usize {
        self.0.push(field_type as u8);
        let idx = self.0.len();
        self.0.extend([0, 0]);
        move |this, size| {
            let len = size as u16;
            this.0[idx] = len.to_be_bytes()[0];
            this.0[idx + 1] = len.to_be_bytes()[1];
            size + 3
        }
    }
    pub fn known_size(&mut self, field_type: FieldType, size: usize) -> usize {
        self.0.push(field_type as u8);
        self.0.extend((size as u16).to_be_bytes());
        3
    }
}

pub trait Serialize {
    fn serialize(&self, buf: &mut Serializer) -> usize;
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut Serializer) -> usize {
        let s = buf.unknown_size(FieldType::Vec);
        let mut size = 0;
        for field in self {
            size += field.serialize(buf);
        }
        s(buf, size)
    }
}

impl Serialize for Field {
    fn serialize(&self, buf: &mut Serializer) -> usize {
        match self {
            Field::Str(s) => {
                buf.known_size(FieldType::Str, s.len());
                buf.0.extend_from_slice(s.as_bytes());
                s.len() + 3
            }
            Field::I128(b) => {
                buf.known_size(FieldType::I128, 16);
                buf.0.extend(b.to_be_bytes());
                16 + 3
            }
            Field::Byte(b) => {
                buf.known_size(FieldType::Byte, 1);
                buf.0.push(*b);
                1 + 3
            }
            Field::Bool(b) => {
                buf.known_size(FieldType::Bool, 1);
                buf.0.push(*b as u8);
                1 + 3
            }
            Field::Action(action) => action.serialize(buf),
            Field::Entity(entity) => entity.serialize(buf),
            Field::Session(session) => session.serialize(buf),
            Field::ActionKind(action_kind) => {
                buf.known_size(FieldType::ActionKind, 1);
                buf.0.push(*action_kind as u8);
                1 + 3
            }
            Field::Vec(values) => values.serialize(buf),
            Field::RealBoolean(_) => {
                buf.known_size(FieldType::Bool, 1);
                buf.0.push(0);
                1 + 3
            }
        }
    }
}

pub trait Deserialize {
    fn deserialize(field_reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized;
}

impl<T, E: Into<Error>> Deserialize for Vec<T>
where
    T: TryFrom<Field, Error = E>,
{
    fn deserialize(field_reader: &mut FieldReader<'_>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();
        while !field_reader.buffer.is_empty() {
            let v: Field = field_reader.read_field()?;
            vec.push(T::try_from(v).map_err(Into::into)?);
        }
        Ok(vec)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldType {
    Str = 1,
    I128,
    Byte,
    Bool,
    Action,
    ActionKind,
    Entity,
    Session,
    Vec,
    RealBoolean,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Field {
    Str(String),
    Byte(u8),
    Bool(bool),
    I128(i128),
    Action(Action),
    ActionKind(ActionKind),
    Entity(Entity),
    Session(Session),
    Vec(Vec<Field>),
    RealBoolean(Boolean),
}

macro_rules! impl_try_from {
    ($type:ty, $field_type:path) => {
        impl TryFrom<Field> for $type {
            type Error = Error;

            fn try_from(value: Field) -> Result<Self> {
                match value {
                    $field_type(val) => Ok(val.into()),
                    _ => Err(Error::InvalidFieldType),
                }
            }
        }
    };
}

impl_try_from!(i128, Field::I128);
impl_try_from!(u8, Field::Byte);
impl_try_from!(bool, Field::Bool);
impl_try_from!(String, Field::Str);
impl_try_from!(Action, Field::Action);
impl_try_from!(ActionKind, Field::ActionKind);
impl_try_from!(Entity, Field::Entity);

impl<T: TryFrom<Field, Error = E>, E: Into<Error>> TryFrom<Field> for Vec<T> {
    type Error = Error;

    fn try_from(value: Field) -> Result<Self, Self::Error> {
        match value {
            Field::Vec(values) => values
                .into_iter()
                .map(T::try_from)
                .collect::<Result<_, _>>()
                .map_err(Into::into),
            _ => Err(Error::InvalidFieldType),
        }
    }
}

pub struct FieldReader<'a> {
    buffer: &'a [u8],
}

impl<'a> FieldReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }

    fn field_type(&mut self) -> Result<FieldType> {
        if self.buffer.is_empty() {
            return Err(Error::MissingFieldType);
        }
        let byte = self.buffer[0];
        self.buffer = &self.buffer[1..];

        log!("Field Type: {:?}", byte);
        match byte {
            1 => Ok(FieldType::Str),
            2 => Ok(FieldType::I128),
            3 => Ok(FieldType::Byte),
            4 => Ok(FieldType::Bool),
            5 => Ok(FieldType::Action),
            6 => Ok(FieldType::ActionKind),
            7 => Ok(FieldType::Entity),
            8 => Ok(FieldType::Session),
            9 => Ok(FieldType::Vec),
            _ => Err(Error::InvalidFieldType),
        }
    }

    fn len(&mut self) -> Result<usize> {
        if self.buffer.len() < 2 {
            return Err(Error::MissingFieldLen);
        }
        let bytes = &self.buffer[..2];
        self.buffer = &self.buffer[2..];
        let len = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(len as usize)
    }

    fn read_be_i128(input: &[u8]) -> i128 {
        let (int_bytes, _) = input.split_at(std::mem::size_of::<i128>());
        i128::from_be_bytes(int_bytes.try_into().unwrap())
    }

    pub fn read_field<T, E>(&mut self) -> Result<T>
    where
        T: TryFrom<Field, Error = E>,
        E: Into<Error>,
    {
        let field_type = self.field_type()?;
        let len = self.len()?;
        
        log!("Field Type parsed: {field_type:?} with length: {len}");
        let bytes = &self.buffer[..len];
        self.buffer = &self.buffer[len..];

        log!("Entity bytes: {bytes:?}");
        log!("Remaining buffer: {:?}", self.buffer);
        let field = match field_type {
            FieldType::Str => Field::Str(std::str::from_utf8(bytes)?.to_owned()),
            FieldType::Bool => Field::Bool(bytes[0] == 1),
            FieldType::Byte => Field::Byte(bytes[0]),
            FieldType::Action => {
                let mut new_reader = FieldReader::new(bytes);
                Field::Action(Action::deserialize(&mut new_reader)?)
            }
            FieldType::Entity => {
                let mut new_reader = FieldReader::new(bytes);
                Field::Entity(Entity::deserialize(&mut new_reader)?)
            }
            FieldType::Session => {
                let mut new_reader = FieldReader::new(bytes);
                Field::Session(Session::deserialize(&mut new_reader)?)
            }
            FieldType::I128 => Field::I128(Self::read_be_i128(bytes)),
            FieldType::ActionKind => Field::ActionKind(unsafe { std::mem::transmute(bytes[0]) }),
            FieldType::Vec => {
                let mut new_reader = FieldReader::new(bytes);
                Field::Vec(Deserialize::deserialize(&mut new_reader)?)
            }
            FieldType::RealBoolean => Field::RealBoolean(Boolean::Maybe),
        };
        field.try_into().map_err(Into::into)
    }
}
