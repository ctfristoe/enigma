use crate::reflector::{Reflector, ReflectorType};

#[test]
fn reflector_a_reflects_a_to_z() {
    let reflector = Reflector::from_type(ReflectorType::A);
    let output = reflector.reflect(0);
    assert_eq!(25, output);
}

#[test]
fn reflector_a_reflects_z_to_a() {
    let reflector = Reflector::from_type(ReflectorType::A);
    let output = reflector.reflect(25);
    assert_eq!(0, output);
}

#[test]
fn reflector_b_reflects_k_to_n() {
    let reflector = Reflector::from_type(ReflectorType::B);
    let output = reflector.reflect(10);
    assert_eq!(13, output);
}

#[test]
fn reflector_c_reflects_q() {
    let reflector = Reflector::from_type(ReflectorType::C);
    let output = reflector.reflect(16);
    assert_eq!(19, output);
}