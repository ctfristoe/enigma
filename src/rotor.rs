use std::convert::From;
use std::str::FromStr;

/// A blueprint for constructing a `Rotor` object.
///
/// Rotors have 26 pins corresponding to letters A-Z, represented here as indexes 0-25.
/// An electrical signal applied to a given pin always exits from a different pin
/// on the oposite face.
///
/// `translation` specifies how pins on opposite faces are connected. a values of b"EK..."
///  means that the first, or 'A', pin is wired to the fifth, or 'E', pin on the opposite face.
///
/// `notches` correcsponds to one or two pin positions along the rotor. When a rotor is rotated
/// from a notch position, it causes the next rotor in the series to move as well.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotorSpec {
    translation: &'static [u8; 26],
    notches: &'static [u8],
}

impl RotorSpec {
    pub const I: Self = Self {
        translation: b"EKMFLGDQVZNTOWYHXUSPAIBRCJ",
        notches: b"Q",
    };
    pub const II: Self = Self {
        translation: b"AJDKSIRUXBLHWTMCQGZNPYFVOE",
        notches: b"E",
    };
    pub const III: Self = Self {
        translation: b"BDFHJLCPRTXVZNYEIWGAKMUSQO",
        notches: b"V",
    };
    pub const IV: Self = Self {
        translation: b"ESOVPZJAYQUIRHXLNFTGKDCMWB",
        notches: b"J",
    };
    pub const V: Self = Self {
        translation: b"VZBRGITYUPSDNHLXAWMJQOFECK",
        notches: b"Z",
    };
    pub const VI: Self = Self {
        translation: b"JPGVOUMFYQBENHZRDKASXLICTW",
        notches: b"ZM",
    };
    pub const VII: Self = Self {
        translation: b"NZJHGRCXMYSWBOUFAIVLPEKQDT",
        notches: b"ZM",
    };
    pub const VIII: Self = Self {
        translation: b"FKQHTLXOCBJSPDZRAMEWNIUYGV",
        notches: b"ZM",
    };
}

impl FromStr for RotorSpec {
    type Err = &'static str;
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Self::I),
            "II" => Ok(Self::II),
            "III" => Ok(Self::III),
            "IV" => Ok(Self::IV),
            "V" => Ok(Self::V),
            "VI" => Ok(Self::VI),
            "VII" => Ok(Self::VII),
            "VIII" => Ok(Self::VIII),
            _ => Err("No corresponding RotorSpec"),
        }
    }
}

/// Represents a rotor that translates input signal to output in a distinct way.
///
/// Rotors have 26 pins corresponding to letters A-Z, represented here as indexes 0-25.
/// An electrical signal applied to a given pin always exits from a different pin
/// on the oposite face.
#[derive(PartialEq, Debug)]
pub struct Rotor {
    r_l_translation: [u8; 26],
    l_r_translation: [u8; 26],
    notch_positions: [u8; 2],
}

impl Rotor {
    /// Accepts one `signal` (an 8-bit integer 0-26) and returns another enciphered output.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right. The
    /// ouput depends on
    pub fn encipher_right_to_left(&self, index: u8) -> u8 {
        self.r_l_translation[index as usize]
    }

    /// Translates a `signal` at one of 26 pins to an output at one of 26 pins.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right.
    pub fn encipher_left_to_right(&self, index: u8) -> u8 {
        self.l_r_translation[index as usize]
    }

    /// Translates a `signal` at one of 26 pins to an output at one of 26 pins.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right.
    pub fn is_notch(&self, index: u8) -> bool {
        self.notch_positions.contains(&index)
    }
}

impl From<RotorSpec> for Rotor {
    /// Constructs a new `Rotor` object from a `RotorSpec` blueprint.
    ///
    /// If anything about `spec` is invalid, this method will panic. It should
    /// only be passed the `RotorSpec` objects enumerated in this file (I-VIII).
    fn from(spec: RotorSpec) -> Self {
        let mut r_l_translation = [u8::MAX; 26];
        let mut l_r_translation = [u8::MAX; 26];
        let mut notch_positions = [u8::MAX; 2];

        for (i, byte) in spec.translation.iter().enumerate() {
            r_l_translation[i] = *byte - 0x41;
            l_r_translation[*byte as usize - 0x41] = i as u8;
        }

        for (i, byte) in spec.notches.iter().enumerate() {
            notch_positions[i] = *byte - 0x41;
        }

        Rotor {
            r_l_translation,
            l_r_translation,
            notch_positions,
        }
    }
}

#[cfg(test)]
mod rotor_tests {
    use test_case::test_case;

    use super::*;

    #[test_case(RotorSpec::I, 0, 4)]
    #[test_case(RotorSpec::II, 1, 9)]
    #[test_case(RotorSpec::III, 1, 3)]
    #[test_case(RotorSpec::IV, 2, 14)]
    #[test_case(RotorSpec::V, 3, 17)]
    #[test_case(RotorSpec::VI, 5, 20)]
    #[test_case(RotorSpec::VII, 8, 12)]
    #[test_case(RotorSpec::VIII, 13, 3)]
    fn rotor_enciphers_right_to_left(spec: RotorSpec, signal: u8, expected: u8) {
        let rotor = Rotor::from(spec);
        let scrambled_signal = rotor.encipher_right_to_left(signal);
        assert_eq!(expected, scrambled_signal);
    }

    #[test_case(RotorSpec::I, 0, 20)]
    #[test_case(RotorSpec::II, 1, 9)]
    #[test_case(RotorSpec::III, 1, 0)]
    #[test_case(RotorSpec::IV, 2, 22)]
    #[test_case(RotorSpec::V, 3, 11)]
    #[test_case(RotorSpec::VI, 5, 7)]
    #[test_case(RotorSpec::VII, 8, 17)]
    #[test_case(RotorSpec::VIII, 13, 20)]
    fn rotor_enciphers_left_to_right(spec: RotorSpec, signal: u8, expected: u8) {
        let rotor = Rotor::from(spec);
        let scrambled_signal = rotor.encipher_left_to_right(signal);
        assert_eq!(expected, scrambled_signal);
    }

    #[test_case(RotorSpec::I, 0, false)]
    #[test_case(RotorSpec::II, 4, true)]
    #[test_case(RotorSpec::III, 21, true)]
    #[test_case(RotorSpec::IV, 10, false)]
    #[test_case(RotorSpec::V, 25, true)]
    #[test_case(RotorSpec::VI, 25, true)]
    #[test_case(RotorSpec::VII, 24, false)]
    #[test_case(RotorSpec::VIII, 12, true)]
    fn rotor_tests_notch_position(spec: RotorSpec, position: u8, expect_notch: bool) {
        let rotor = Rotor::from(spec);
        let is_notch = rotor.is_notch(position);
        assert_eq!(expect_notch, is_notch);
    }
}
