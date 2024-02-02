use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

use crate::actions::Action;
use crate::error::{Error, Result};
use crate::serde::{serialize, Deserialize, FieldReader, FieldType, Serialize, Serializer};
use crate::strings::{self, Boolean};
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
        let value = &extr(value).chars().take(15).collect::<String>();
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
        table_row(f, 15, &self.party, |o| o.name.to_string())?;
        table_row(f, 15, &self.party, |o| {
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
        "Lol - How good are your booleans??";
        if !matches!(strings::rand(), Boolean::Luck(_)) {
            return Err(Error::InvalidArgs("


            Lorem chad ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. At urna condimentum mattis pellentesque id nibh tortor id aliquet. Convallis convallis tellus id interdum velit laoreet id donec. Magnis dis parturient montes nascetur ridiculus mus. Posuere ac ut consequat semper viverra nam. A iaculis at erat pellentesque adipiscing commodo. Ut porttitor leo a diam sollicitudin tempor id. Dolor purus non enim praesent elementum facilisis leo vel. Neque egestas congue quisque egestas. Diam volutpat commodo sed egestas egestas fringilla phasellus faucibus. Eget arcu dictum varius duis at consectetur lorem. Magna fermentum iaculis eu non diam phasellus vestibulum. Dictumst quisque sagittis purus sit amet volutpat consequat. Id donec ultrices tincidunt arcu. Ut tristique et egestas quis ipsum suspendisse ultrices gravida. Non quam lacus suspendisse faucibus interdum posuere lorem. Tristique nulla aliquet enim tortor. Commodo elit at imperdiet dui accumsan. Duis at consectetur lorem donec.
            Enim praesent elementum facilisis leo vel fringilla est. At varius vel pharetra vel turpis nunc eget. Lectus magna fringilla urna porttitor. Egestas congue quisque egestas diam in arcu. Velit aliquet sagittis id consectetur purus ut faucibus pulvinar. Non tellus orci ac auctor. At risus viverra adipiscing at in tellus integer. Phasellus egestas tellus rutrum tellus pellentesque. Eget sit amet tellus cras adipiscing enim. Et egestas quis ipsum suspendisse. Scelerisque in dictum non consectetur a erat nam at.
            Donec ultrices tincidunt arcu non sodales. Diam phasellus vestibulum lorem sed risus ultricies. Nunc congue nisi vitae suscipit tellus mauris. Elementum nisi quis eleifend quam adipiscing vitae proin sagittis. Platea dictumst quisque sagittis purus sit amet. Sit amet nisl purus in mollis. Praesent semper feugiat nibh sed pulvinar. Tortor condimentum lacinia quis vel eros donec. Condimentum lacinia quis vel eros. Fames ac turpis egestas integer eget aliquet nibh praesent tristique.
            Vel quam elementum pulvinar etiam non. Duis ut diam quam nulla porttitor massa id neque. Lacinia at quis risus sed vulputate odio ut enim. Dui sapien eget mi proin sed libero enim sed faucibus. Et pharetra pharetra massa massa ultricies. Senectus et netus et malesuada. In tellus integer feugiat scelerisque. Senectus et netus et malesuada fames ac turpis egestas maecenas. Ut tortor pretium viverra suspendisse. Lobortis feugiat vivamus at augue eget arcu dictum. Ut enim blandit volutpat maecenas volutpat blandit aliquam etiam. Vestibulum sed arcu non odio euismod. Cursus mattis molestie a iaculis. Porttitor rhoncus dolor purus non. Tempus quam pellentesque nec nam aliquam sem et tortor. Euismod elementum nisi quis eleifend quam.
            Gravida cum sociis natoque penatibus et magnis dis. Laoreet non curabitur gravida arcu. Viverra suspendisse potenti nullam ac tortor vitae purus faucibus ornare. Commodo elit at imperdiet dui accumsan sit amet nulla facilisi. Ipsum a arcu cursus vitae. Felis eget velit aliquet sagittis id consectetur. Lectus nulla at volutpat diam ut venenatis tellus. Nisl vel pretium lectus quam id. Sapien pellentesque habitant morbi tristique senectus. Diam sit amet nisl suscipit. Ante metus dictum at tempor commodo ullamcorper a. Neque laoreet suspendisse interdum consectetur libero. Pellentesque dignissim enim sit amet venenatis urna cursus eget nunc. Blandit volutpat maecenas volutpat blandit aliquam etiam. Id interdum velit laoreet id donec ultrices tincidunt. Turpis in eu mi bibendum neque egestas congue quisque.
            Tellus pellentesque eu tincidunt tortor aliquam nulla facilisi cras fermentum. Enim facilisis gravida neque convallis a cras. Vitae aliquet nec ullamcorper sit amet risus. Duis at tellus at urna condimentum mattis pellentesque. Cras pulvinar mattis nunc sed. Ac turpis egestas sed tempus urna et pharetra pharetra massa. Ultrices eros in cursus turpis massa tincidunt dui. Massa sed elementum tempus egestas sed. Nisi scelerisque eu ultrices vitae auctor eu augue. Id neque aliquam vestibulum morbi. Sapien pellentesque habitant morbi tristique senectus et netus et. Tellus in metus vulputate eu. Feugiat in fermentum posuere urna nec tincidunt praesent. Gravida quis blandit turpis cursus. Scelerisque fermentum dui faucibus in ornare quam.
            Sit amet dictum sit amet. Porta lorem mollis aliquam ut porttitor leo. Sit amet consectetur adipiscing elit duis tristique sollicitudin nibh sit. Felis bibendum ut tristique et. In hendrerit gravida rutrum quisque non. Ullamcorper velit sed ullamcorper morbi tincidunt ornare. Commodo quis imperdiet massa tincidunt. Faucibus turpis in eu mi bibendum neque egestas congue quisque. Integer enim neque volutpat ac tincidunt. Euismod quis viverra nibh cras pulvinar mattis. Facilisis magna etiam tempor orci eu. Senectus et netus et malesuada fames ac turpis egestas integer. Mi ipsum faucibus vitae aliquet nec. Ullamcorper morbi tincidunt ornare massa eget egestas purus viverra accumsan. Placerat orci nulla pellentesque dignissim enim sit amet venenatis.
            Amet massa vitae tortor condimentum lacinia quis vel eros. Tellus in hac habitasse platea dictumst vestibulum. Quisque egestas diam in arcu cursus. Sit amet consectetur adipiscing elit pellentesque habitant morbi tristique senectus. Eleifend donec pretium vulputate sapien nec sagittis aliquam malesuada bibendum. Sit amet consectetur adipiscing elit pellentesque habitant morbi tristique senectus. Ut ornare lectus sit amet est placerat in egestas erat. Consectetur purus ut faucibus pulvinar elementum. Senectus et netus et malesuada fames ac turpis. Proin libero nunc consequat interdum varius sit amet mattis. Aliquet bibendum enim facilisis gravida neque convallis a cras. Sapien eget mi proin sed libero enim sed faucibus. Mi bibendum neque egestas congue. Molestie at elementum eu facilisis sed odio morbi quis commodo. Nibh tortor id aliquet lectus proin nibh. Varius quam quisque id diam vel.


            Caught exception: map::at, trace:
            0# get_data_from_config(std::string_view) at /home/U/basic.cpp:420
            1# bar(std::string_view) at /home/U/basic.cpp:69
            2# main at /home/U/basic.cpp:17



            Segmentation fault 
            "));
        }

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
        let mut reader = FieldReader::new(&bytes.as_slice()[3..]);
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
        serde::{serialize, Deserialize, FieldReader},
        Entity,
    };

    use super::Session;

    fn deserialize<T: Deserialize>(bytes: &[u8]) -> crate::error::Result<T> {
        let mut reader = FieldReader::new(&bytes[3..]);
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
}
