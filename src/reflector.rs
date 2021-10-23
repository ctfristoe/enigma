
use crate::settings::;

#[cfg(test)]
#[path = "./tests/reflector.rs"]
mod tests;

#[derive(Debug)]
pub enum ReflectorType {
    A, B, C
}

#[derive(Debug)]
pub struct Reflector {
    wiring: [u32; 26]
}

impl Reflector {
    pub fn from_type(reflector_type: ReflectorType) -> Self {
        match reflector_type {
            ReflectorType::A => Reflector::new("ZYXWVUTSRQPONMLKJIHGFEDCBA"),
            ReflectorType::B => Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            ReflectorType::C => Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
        }
    }

    pub fn reflect(&self, input: u32) -> u32 {
        self.wiring[input as usize]
    }

    fn new(specification: &str) -> Self {       
        assert!(specification.len() == 26, "invalid rotor specification");
        let mut wiring = [u32::MAX; 26];
        for (i, letter) in specification.chars().enumerate() {
            wiring[i] = letter as u32 - 65;
        }
        Reflector { wiring }
    }
}