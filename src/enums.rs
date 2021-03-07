use std::str::FromStr;

// Ideally i'd like to change this to be a macro,
// to do implementations for both FromStr and From<T>.

#[derive(Debug)]
pub(crate) enum OpusApplication {
    Audio,
    Voip,
    LowDelay
}

impl FromStr for OpusApplication {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(Self::Audio),
            "voip" => Ok(Self::Voip),
            "lowdelay" => Ok(Self::LowDelay),
            _ => Err("unknown application type"),
        }
    }
}

impl ToString for OpusApplication {
    fn to_string(&self) -> String {
        match self {
            Self::Audio => String::from("audio"),
            Self::Voip => String::from("voip"),
            Self::LowDelay => String::from("lowdelay"),
        }
    }
}

impl From<OpusApplication> for opus::Application {
    fn from(x: OpusApplication) -> Self {
        match x {
            OpusApplication::Audio => Self::Audio,
            OpusApplication::Voip => Self::Voip,
            OpusApplication::LowDelay => Self::LowDelay,
        }
    }
}
