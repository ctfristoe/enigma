use std::fmt::{self, Display, Formatter};
use std::convert::{From, TryFrom};


#[derive(Copy, Clone)]
pub struct Letter {
    byte: u8
}

impl From<Letter> for u8 {
    fn from(value: Letter) -> u8 {
        value.byte
    }
}

impl From<Letter> for u32 {
    fn from(value: Letter) -> u32 {
        value.byte as u32
    }
}

impl From<Letter> for usize {
    fn from(value: Letter) -> usize {
        value.byte as usize
    }
}

impl From<Letter> for char {
    fn from(value: Letter) -> char {
        (value.byte + 65) as char
    }
}

impl TryFrom<u8> for Letter {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 25 {
            Err(format!("cannot cast value {} to Letter", value))
        } else {
            Ok(Letter { byte: value })
        }
    }
}

impl TryFrom<u32> for Letter {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Letter::try_from(value as u8)
    }
}

impl TryFrom<usize> for Letter {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Letter::try_from(value as u8)
    }
}

impl TryFrom<char> for Letter {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let as_int = value as u8 - 65;
        match Letter::try_from(as_int) {
            Ok(result) => Ok(result),
            Err(_) => Err(format!("cannot cast value {} to Letter", value))
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let as_char: char = self.clone().into();
        as_char.fmt(f)
    }
}

