use crate::letter::Letter;
use crate::plugboard::Plugboard;
use crate::reflector::{Reflector, ReflectorType};
use crate::rotor::{Rotor, RotorType};

#[derive(Debug)]
struct Enigma {
    l_rotor: Rotor,
    m_rotor: Rotor,
    r_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}


impl Enigma {

    fn new(
        l_rotor_type: RotorType,
        m_rotor_type: RotorType,
        r_rotor_type: RotorType,
        reflector_type: ReflectorType,
    ) -> Self {
        
    }
}