use std::convert::{From, TryFrom};

use crate::Letter;
use crate::RotorType;

#[cfg(test)]
#[path = "./tests/rotor.rs"]
mod tests;

// Rotors differ in terms of internal wiring and notch location(s).

// Rotors have positions A-Z (printed on the side), represented here as indexes 0-25.
// An electrical signal applied to a given position always exits from a different position
// on the oposite face. A 'specification' of 'EK...' means that position 'A' is wired to
// poition 'E' on the opposite face, 'B' to 'K', and so on. 

// 3 rotors are connected in a series. The right rotor turns on every key press. Whenever
// a rotor moves from its notch position to the next position, it turns the next rotor in 
// the series by one position.

#[derive(Debug)]
pub struct Rotor {
    forward_wiring: [u32; 26],
    reverse_wiring: [u32; 26],
    notch_positions: [u32; 2],
}

impl Rotor {
    fn new(specification: &str, notches: &str) -> Self {
        assert!(specification.len() == 26, "invalid specification");
        assert!([1, 2].contains(&notches.len()), "invalid notches");

        let mut forward_wiring = [u32::MAX; 26];
        let mut reverse_wiring = [u32::MAX; 26];
        let mut notch_positions = [u32::MAX; 2];
        
        for (i, c) in specification.chars().enumerate() {
            let letter = Letter::try_from(c).unwrap();
            forward_wiring[i] = letter.into();
            reverse_wiring[letter.into()] = i as u32;
        }

        for (i, letter) in notches.chars().enumerate() {
            notch_positions[i] = letter as u32 - 65;
        }
        
        Rotor { forward_wiring, reverse_wiring, notch_positions }
    }
    
    pub fn forward(&self, input: u32) -> u32 {
        self.forward_wiring[input as usize]
    }

    pub fn reverse(&self, input: u32) -> u32 {
        self.reverse_wiring[input as usize]
    }
    
    pub fn is_notch(&self, position: u32) -> bool {
        self.notch_positions.contains(&position)
    }
}

impl From<RotorType> for Rotor {
    fn from(rotor_type: RotorType) -> Self {
        match rotor_type {
            RotorType::I    => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "Q"),
            RotorType::II   => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", "E"),
            RotorType::III  => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", "V"),
            RotorType::IV   => Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", "J"),
            RotorType::V    => Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", "Z"),
            RotorType::VI   => Rotor::new("JPGVOUMFYQBENHZRDKASXLICTW", "ZM"),
            RotorType::VII  => Rotor::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", "ZM"),
            RotorType::VIII => Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", "ZM"),
        }
    }
}