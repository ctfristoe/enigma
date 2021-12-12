use std::convert::From;

use crate::ReflectorSpec;
use crate::RotorSpec;


#[derive(PartialEq, Debug)]
/// Represents a reflector that translates an input index to a different output index.
///
/// Like the rotors, the reflector accepts a index at one of 26 pins and always outputs
/// index from a different pin. Unlike the rotors, the reflector inputs and outputs index
/// on the same face. Because of this, the reflector always switches si
pub struct Reflector {
    translation: [u8; 26],
}

impl Reflector {
    pub fn encipher(&self, index: u8) -> u8 {
        self.translation[index as usize]
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

#[derive(PartialEq, Debug)]
/// Represents three rotors arranged in a series, with variable positions.
///
/// The 'position' of the rotor refers to which of the 26 pins are facing directly up.
/// The relative positions of the rotors when the index is sent to the first rotor vary
/// the output that is returned. When the rotor moves from a notch position to the next
/// position, it turns the next rotor in the series by one position.
pub struct RotorBank {
    pub left: Rotor,
    pub right: Rotor,
    pub middle: Rotor,
}

impl RotorBank {
    pub fn encipher_right_to_left(&self, index: u8) -> u8 {
        let index = self.right.encipher_right_to_left(index);
        let index = self.middle.encipher_right_to_left(index);
        let index = self.left.encipher_right_to_left(index);
        index
    }
    pub fn encipher_left_to_right(&self, index: u8) -> u8 {
        let index = self.left.encipher_left_to_right(index);
        let index = self.middle.encipher_left_to_right(index);
        let index = self.right.encipher_left_to_right(index);
        index
    }
    pub fn turn(&mut self) {
        let _ = self.left.turn() && self.middle.turn() && self.right.turn();
    }

    pub fn set_positions(&mut self, left: u8, right: u8, middle: u8) {
        self.left.set_position(left);
        self.right.set_position(right);
        self.middle.set_position(middle)
    }
}

#[derive(PartialEq, Debug)]
/// Represents a rotor that translates input signal to output in a distinct way.
///
/// Rotors have 26 pins corresponding to letters A-Z, represented here as indexes 0-25.
/// An electrical signal applied to a given pin always exits from a different pin
/// on the oposite face.
pub struct Rotor {
    r_l_translation: [u8; 26],
    l_r_translation: [u8; 26],
    notch_positions: [u8; 2],
    position: u8,
}

impl Rotor {
    /// Accepts one `signal` (an 8-bit integer 0-26) and returns another enciphered output.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right. The
    /// ouput depends on
    fn encipher_right_to_left(&self, index: u8) -> u8 {
        self.r_l_translation[self.get_relative_index(index) as usize]
    }

    /// Translates a `signal` at one of 26 pins to an output at one of 26 pins.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right.
    fn encipher_left_to_right(&self, index: u8) -> u8 {
        self.l_r_translation[self.get_relative_index(index) as usize]
    }

    /// Translates a `signal` at one of 26 pins to an output at one of 26 pins.
    ///
    /// In engima, signals first pass right to left through rotors, then left to right.
    fn turn(&mut self) -> bool {
        let should_propogate = self.notch_positions.contains(&self.position);
        self.position = self.get_relative_index(1);
        should_propogate
    }

    fn set_position(&mut self, position: u8) {
        self.position = position;
    }

    fn get_relative_index(&self, index: u8) -> u8 {
        (self.position + index) % 26
    }
}

impl From<RotorSpec> for Rotor {
    /// Constructs a new `Rotor` object from a `RotorSpec` blueprint.
    ///
    /// If anything about `spec` is invalid, this method will panic. It should
    /// only be passed enumerated `RotorSpec` objects (I-VIII).
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
            position: 0,
        }
    }
}

#[cfg(test)]
mod component_tests {
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
    fn reflector_enciphers(spec: ReflectorSpec, index: u8, expected: u8) {
        let reflector = Reflector::from(spec);
        let output = reflector.encipher(index);
        assert_eq!(expected, output);
    }

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
    fn rotor_propogates_when_turned_from_notch_position(
        spec: RotorSpec, position: u8, expect_propogation: bool
    ) {
        let mut rotor = Rotor::from(spec);
        rotor.set_position(position);
        assert_eq!(expect_propogation, rotor.turn());
    }
}
