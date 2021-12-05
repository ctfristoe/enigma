use crate::reflector::Reflector;
use crate::rotor::Rotor;

/// Represents three rotors arranged in a series, with variable positions.
///
/// The 'position' of the rotor refers to which of the 26 pins are facing directly up.
/// The relative positions of the rotors when the index is sent to the first rotor vary
/// the output that is returned. When the rotor moves from a notch position to the next
/// position, it turns the next rotor in the series by one position.
pub struct EnigmaMachine {
    reflector: Reflector,
    l_rotor: Rotor,
    m_rotor: Rotor,
    r_rotor: Rotor,
    l_position: u8,
    m_position: u8,
    r_position: u8,
}

impl EnigmaMachine {
    pub fn encipher(&mut self, key: u8) -> u8 {
        let mut index = key - 0x41;
        index = self.right_rotor_right_to_left(index);
        index = self.middle_rotor_right_to_left(index);
        index = self.left_rotor_right_to_left(index);
        index = self.reflector.encipher(index);
        index = self.left_rotor_left_to_right(index);
        index = self.middle_rotor_left_to_right(index);
        index = self.right_rotor_left_to_right(index);
        self.turn_rotors();
        index + 0x41
    }

    fn left_rotor_right_to_left(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.l_position);
        self.l_rotor.encipher_right_to_left(index)
    }

    fn left_rotor_left_to_right(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.l_position);
        self.l_rotor.encipher_left_to_right(index)
    }

    fn middle_rotor_right_to_left(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.m_position);
        self.m_rotor.encipher_right_to_left(index)
    }

    fn middle_rotor_left_to_right(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.m_position);
        self.m_rotor.encipher_left_to_right(index)
    }

    fn right_rotor_right_to_left(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.r_position);
        self.r_rotor.encipher_right_to_left(index)
    }

    fn right_rotor_left_to_right(&self, index: u8) -> u8 {
        let index = get_rotor_index(index, self.r_position);
        self.r_rotor.encipher_left_to_right(index)
    }

    fn turn_rotors(&mut self) {
        if self.m_rotor.is_notch(self.m_position) {
            self.l_position = get_rotor_index(1, self.l_position)
        }
        if self.r_rotor.is_notch(self.r_position) {
            self.m_position = get_rotor_index(1, self.m_position);
        }
        self.r_position = get_rotor_index(1, self.m_position);
    }

    pub fn new(
        reflector: Reflector,
        l_rotor: Rotor,
        m_rotor: Rotor,
        r_rotor: Rotor,
        l_position: u8,
        m_position: u8,
        r_position: u8,
    ) -> Result<Self, &'static str> {
        if l_rotor == m_rotor {
            return Err("Left and middle rotors cannot be the same");
        }
        if l_rotor == r_rotor {
            return Err("Left and right rotors cannot be the same");
        }
        if l_rotor == r_rotor {
            return Err("Left and right rotors cannot be the same");
        }
        Ok(Self {
            reflector,
            l_rotor,
            m_rotor,
            r_rotor,
            l_position,
            m_position,
            r_position,
        })
    }
}

fn get_rotor_index(absolute_index: u8, rotor_position: u8) -> u8 {
    (absolute_index + rotor_position) % 26
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ReflectorSpec, RotorSpec};
    use test_case::test_case;

    #[test_case(configuration1, b'A', b'U')]
    #[test_case(configuration1, b'B', b'E')]
    #[test_case(configuration1, b'C', b'J')]
    #[test_case(configuration1, b'D', b'O')]
    #[test_case(configuration1, b'F', b'T')]
    #[test_case(configuration1, b'G', b'P')]
    #[test_case(configuration1, b'H', b'Z')]
    #[test_case(configuration1, b'I', b'W')]
    #[test_case(configuration1, b'K', b'N')]
    #[test_case(configuration1, b'L', b'S')]
    #[test_case(configuration1, b'M', b'R')]
    #[test_case(configuration1, b'Q', b'V')]
    #[test_case(configuration1, b'X', b'Y')]
    fn can_encipher_first_byte(configure: fn() -> EnigmaMachine, input: u8, expected: u8) {
        let output = configure().encipher(input);

        assert_eq!(expected as char, output as char);
    }

    fn configuration1() -> EnigmaMachine {
        EnigmaMachine::new(
            ReflectorSpec::B.into(),
            RotorSpec::I.into(),
            RotorSpec::II.into(),
            RotorSpec::III.into(),
            0,
            0,
            0,
        )
        .unwrap()
    }
}
