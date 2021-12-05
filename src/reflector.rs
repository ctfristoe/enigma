use std::convert::From;
use std::str::FromStr;

/// A blueprint for constructing a `Reflector` object.
///
/// Reflectors have 26 pins corresponding to letters A-Z, like rotors, represented here as
/// indexes 0-25.  An electrical signal applied to a given pin always exits from a different
/// pin on the same face.  Translation through a reflector is therefore always symmetrical
/// and no input can translate to itself.
///
/// `translation` specifies how pins on opposite faces are connected. a values of b"EK..."
///  means that the first, or 'A', translates to the fifth, or 'E', pin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReflectorSpec {
    translation: &'static [u8; 26],
}

impl ReflectorSpec {
    pub const A: Self = Self {
        translation: b"ZYXWVUTSRQPONMLKJIHGFEDCBA",
    };
    pub const B: Self = Self {
        translation: b"YRUHQSLDPXNGOKMIEBFZCWVJAT",
    };
    pub const C: Self = Self {
        translation: b"FVPJIAOYEDRZXWGCTKUQSBNMHL",
    };
}

impl FromStr for ReflectorSpec {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            _ => Err("No corresponding ReflectorSpec"),
        }
    }
}

/// Represents a reflector that translates an input signal to a different output signal.
///
/// Like the rotors, the reflector accepts a signal at one of 26 pins and always outputs
/// signal from a different pin. Unlike the rotors, the reflector inputs and outputs signal
/// on the same face. Because of this, the reflector always switches si
#[derive(PartialEq, Debug)]
pub struct Reflector {
    translation: [u8; 26],
}

impl Reflector {
    pub fn encipher(&self, signal: u8) -> u8 {
        self.translation[signal as usize]
    }
}

impl From<ReflectorSpec> for Reflector {
    fn from(spec: ReflectorSpec) -> Self {
        let mut translation = [u8::MAX; 26];
        for (i, byte) in spec.translation.iter().enumerate() {
            translation[i] = *byte - 0x41;
        }
        Reflector { translation }
    }
}

#[cfg(test)]
mod test_reflector {
    use test_case::test_case;

    use super::*;

    #[test_case(ReflectorSpec::A, 0, 25)]
    #[test_case(ReflectorSpec::A, 25, 0)]
    #[test_case(ReflectorSpec::B, 1, 17)]
    #[test_case(ReflectorSpec::C, 1, 21)]
    #[test_case(ReflectorSpec::B, 2, 20)]
    #[test_case(ReflectorSpec::C, 3, 9)]
    #[test_case(ReflectorSpec::B, 5, 18)]
    #[test_case(ReflectorSpec::C, 8, 4)]
    #[test_case(ReflectorSpec::B, 13, 10)]
    #[test_case(ReflectorSpec::C, 21, 1)]
    fn reflector_enciphers(spec: ReflectorSpec, signal: u8, expected: u8) {
        let reflector = Reflector::from(spec);
        let output = reflector.encipher(signal);
        assert_eq!(expected, output);
    }
}
