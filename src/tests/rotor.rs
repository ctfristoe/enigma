use crate::rotor::{Rotor, RotorType};

#[test]
fn rotor_i_a_to_e_forward() {
    assert_output_forward(RotorType::I, 0, 4);
}

#[test]
fn rotor_i_e_to_a_reverse() {
    assert_output_reverse(RotorType::I, 4, 0);
}

#[test]
fn rotor_ii_z_to_e_forward() {
    assert_output_forward(RotorType::II, 25, 4);
}

#[test]
fn rotor_ii_e_to_z_reverse() {
    assert_output_reverse(RotorType::II, 4, 25);
}

#[test]
fn rotor_iii_no_notch_at_a() {
    assert_no_notch(RotorType::III, 0);
}

#[test]
fn rotor_iv_has_notch_at_j() {
    assert_notch(RotorType::IV, 9);
}

#[test]
fn rotor_vi_has_two_notches() {
    assert_notch(RotorType::VI, 12);
    assert_notch(RotorType::VI, 25);
}

fn assert_output_forward(rtype: RotorType, input: u32, expected: u32) {
    let rotor = Rotor::from_type(rtype);
    let output = rotor.forward(input);
    assert_eq!(expected, output);
}

fn assert_output_reverse(rtype: RotorType, input: u32, expected: u32) {
    let rotor = Rotor::from_type(rtype);
    let output = rotor.reverse(input);
    assert_eq!(expected, output);
}

fn assert_notch(rtype: RotorType, position: u32) {
    let rotor = Rotor::from_type(rtype);
    assert!(rotor.is_notch(position));
}

fn assert_no_notch(rtype: RotorType, position: u32) {
    let rotor = Rotor::from_type(rtype);
    assert!(!rotor.is_notch(position));
}
