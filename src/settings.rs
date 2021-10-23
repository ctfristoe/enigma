use strum_macros::EnumString;

use crate::letter::Letter;

// Reflector A reflects letters symetrically across the alphabet. Reflectors  B and C 
// were usually used.

#[derive(Debug, PartialEq, EnumString)]
pub enum ReflectorType {
    A, B, C
}


// By 1941, German army enigma machines were provisioned with 8 types of rotors
// For simplicity, we're not handling naval or commercial enigma machines in this project

#[derive(Debug, PartialEq, EnumString)]
pub enum RotorType {
    I, II, III, IV, V, VI, VII, VIII
}

#[derive(Debug)]
pub struct Settings {
    l_rotor: RotorType,
    m_rotor: RotorType,
    r_rotor: RotorType,
    l_initial_position: Letter,
    m_initial_position: Letter,
    r_initial_position: Letter,
    reflector_type: ReflectorType,
    plug_board_pairs: Vec<(Letter, Letter)>,
}    

impl Settings {
    fn new() -> Self {
        Self {
            l_rotor: RotorType::I,
            m_rotor: RotorType::II,
            r_rotor: RotorType::III,
            l_initial_position: Letter::from('A'),
            m_initial_position: Letter::from('A'),
            r_initial_position: Letter::from('A'),
            reflector_type: ReflectorType::A,
            plug_board_pairs: vec!(),
        }    
    }
}    
