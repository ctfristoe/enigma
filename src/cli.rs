use std::fs::File;
use std::io::{Read, Write};
use std::io::{stdin, stdout};
use std::path::PathBuf;

use structopt::StructOpt;

use crate::ReflectorSpec;
use crate::RotorSpec;

/// encrypt the file using the enigma algorithm
#[derive(StructOpt, Debug)]
#[structopt(name = "Enigma", about = "A command-line implementation of the engima cryptographic machine.")]
pub struct Cli {
    /// input file, stdin in not present
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
    /// ouptut file, stdout in not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
    /// left rotor type, default is B
    #[structopt(default_value = "B")]
    pub reflector: ReflectorSpec,
    /// left rotor type, default is I
    #[structopt(default_value = "I")]
    pub left_rotor: RotorSpec,
    /// middle rotor type, default is II
    #[structopt(default_value = "II")]
    pub middle_rotor: RotorSpec,
    /// right rotor type, default is III
    #[structopt(default_value = "III")]
    pub right_rotor: RotorSpec,
    /// left rotor initial position, default is A
    #[structopt(default_value = "A")]
    pub left_rotor_position: char,
    /// middle rotor initial position, default is A
    #[structopt(default_value = "A")]
    pub middle_rotor_position: char,
    /// right rotor initial position, default is A
    #[structopt(default_value = "A")]
    pub right_rotor_position: char,
}


impl Cli {
    pub fn validate_args(&self) {
        if self.left_rotor == self.right_rotor {
            panic!("Left rotor and right rotor cannot be the same.")
        }
        if self.left_rotor == self.middle_rotor {
            panic!("Left rotor and middle rotor cannot be the same.");
        }
        if self.right_rotor == self.middle_rotor {
            panic!("Right rotor and middle rotor cannot be the same.");
        }
        if !self.left_rotor_position.is_ascii_uppercase() {
            panic!("Left rotor position must be A-Z.");
        }
        if !self.right_rotor_position.is_ascii_uppercase() {
            panic!("Right rotor position must be A-Z.");
        }
        if !self.middle_rotor_position.is_ascii_uppercase() {
            panic!("Middle rotor position must be A-Z.");
        }
    }

    pub fn get_reader(&self) -> Box<dyn Read> {
        match &self.input {
            Some(path) => {
                let file = File::open(path).unwrap();
                Box::new(file)
                // let reader = BufReader::new(file);
            },
            None => {
                // let reader = BufReader::new(stdin());
                Box::new(stdin())
            },
        }
    }

    pub fn get_writer(&self) -> Box<dyn Write> {
        match &self.output {
            Some(path) => {
                let file = File::create(path).unwrap();
                Box::new(file)
                // let writer = BufWriter::new(file);
                
            },
            None => {
                Box::new(stdout())
            },
        }
    }
}