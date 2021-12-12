use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A blueprint for constructing a `Reflector` object.
///
/// Reflectors have 26 pins corresponding to letters A-Z, like rotors, represented here as
/// indexes 0-25.  An electrical index applied to a given pin always exits from a different
/// pin on the same face.  Translation through a reflector is therefore always symmetrical
/// and no input can translate to itself.
///
/// `translation` specifies how pins on opposite faces are connected. a values of b"EK..."
///  means that the first, or 'A', translates to the fifth, or 'E', pin.
pub struct ReflectorSpec {
    pub translation: &'static [u8; 26],
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
    type Err = String;
    /// Returns a valid `RotorSpec` from a letter A-C, or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            _ => Err(format!("invalid reflector type {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct RotorSpec {
    pub translation: &'static [u8; 26],
    pub notches: &'static [u8],
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
    type Err = String;
    /// Returns a valid `RotorSpec` from a roman numeral I-VIII, or an error.
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
            _ => Err(format!("invalid rotor type {}", s)),
        }
    }
}