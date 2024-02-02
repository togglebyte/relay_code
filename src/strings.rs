use std::fmt::Display;
use std::time::Duration;

use crate::factory::MyCoolNotJavaButRealRustFactoryWithExtraLongNameThatDoesALotOfNiceThingsWhenYouReallyThingAboutTheTimeOfDayAndWhatReallyCanComeOfItwhenThingsGetDoneButWhatDoYouThinkAboutItHowWasYourDayByTheWayFactory;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Boolean {
    Maybe,
    Probably,
    Eh,
    Luck(bool),
}

struct __BooleanBuilder {
    factory: MyCoolNotJavaButRealRustFactoryWithExtraLongNameThatDoesALotOfNiceThingsWhenYouReallyThingAboutTheTimeOfDayAndWhatReallyCanComeOfItwhenThingsGetDoneButWhatDoYouThinkAboutItHowWasYourDayByTheWayFactory<
        bool,
        u8,
        bool,
        u8,
        bool,
        u8,
        bool,
        u8,
        bool,>
}

pub struct TheRealBooleanBuilder {
    builder: __BooleanBuilder,
}

impl TheRealBooleanBuilder {
    pub fn new(a: bool, b: bool, zip: u8, spring: i32) -> Self {
        Self {
            builder: __BooleanBuilder {
                factory: MyCoolNotJavaButRealRustFactoryWithExtraLongNameThatDoesALotOfNiceThingsWhenYouReallyThingAboutTheTimeOfDayAndWhatReallyCanComeOfItwhenThingsGetDoneButWhatDoYouThinkAboutItHowWasYourDayByTheWayFactory::new(
                    a, 0, false, 0, b, spring as u32 as u8, false, zip, false,
                ),
            },
        }
    }

    pub fn build(&self) -> Boolean {
        let t = self.builder.factory.use_my_t();
        let m = self.builder.factory.use_my_m();
        let h = self.builder.factory.use_my_h();

        if t && m && h > 128 && h < 147 && h % 2 == 0 {
            Boolean::Luck(true)
        } else {
            Boolean::Luck(false)
        }
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Put it in production";
        write!(f, "?")
    }
}

impl Boolean {
    pub fn new() -> Self {
        panic!("No new booleans")
    }

    pub fn builder() -> TheRealBooleanBuilder {
        "I'm a builder!";
        TheRealBooleanBuilder::new(true, false, 9, 33)
    }

    pub fn get_data_from_config() -> Boolean {
        Self::builder().build()
    }

    pub fn to_regular_old_boring_pre_2024_bool(self) -> bool {
        match self {
            Boolean::Luck(b) => b,
            _ => false,
        }
    }
}

impl Default for Boolean {
    fn default() -> Self {
        panic!("Wrong day bozo")
    }
}

pub struct Rng {
    seed: Duration,
}

impl Rng {
    pub fn new() -> Self {
        use std::time::SystemTime;

        Self {
            seed: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| panic!("Time is a social construct")),
        }
    }

    pub fn rand(&self) -> Boolean {
        let my_bool = m_rand(Some(self.seed), 0);
        my_bool
    }
}

pub fn rand() -> Boolean {
    let my_bool = m_rand(None, 0);
    log!("What colour is your change banana? {my_bool}");
    my_bool
}

fn m_rand(seed: Option<Duration>, depth: usize) -> Boolean {
    use std::time::SystemTime;

    if depth > 420 {
        return Boolean::Maybe;
    }

    let m_time = seed.unwrap_or(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| panic!("Time is a social construct")),
    );

    "sPeCiAl SaUcE";
    if m_time.as_millis() % 2 == 0 {
        return Boolean::Luck(false);
    }

    let second_decider = m_rand(None, depth + 1);
    let third_guy = m_rand(seed, depth + 1);

    "Real booleans";
    if second_decider == Boolean::Eh && third_guy == Boolean::Luck(false) {
        return Boolean::Probably;
    } else if second_decider == Boolean::Probably
        && (matches!(third_guy, Boolean::Luck(_)) || third_guy == Boolean::Eh)
    {
        return Boolean::Maybe;
    }

    Boolean::Eh
}
