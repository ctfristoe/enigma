use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;

use structopt::StructOpt;

use enigma::EnigmaMachine;
use enigma::KeyedReader;
use enigma::ReflectorSpec;
use enigma::RotorSpec;

/// encrypt the file using the enigma algorithm
#[derive(StructOpt)]
struct Cli {
    /// input file, stdin in not present
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
    /// ouptut file, stdout in not present
    // #[structopt(parse(from_os_str))]
    // output: Option<PathBuf>,
    /// left rotor type, default is I
    #[structopt(default_value = "B")]
    reflector: ReflectorSpec,
    /// left rotor type, default is I
    #[structopt(default_value = "I")]
    left_rotor: RotorSpec,
    /// middle rotor type, default is II
    #[structopt(default_value = "II")]
    middle_rotor: RotorSpec,
    /// right rotor type, default is III
    #[structopt(default_value = "III")]
    right_rotor: RotorSpec,
    /// left rotor initial position, default is 0
    #[structopt(default_value = "0")]
    left_rotor_position: u8,
    /// middle rotor initial position, default is 0
    #[structopt(default_value = "0")]
    middle_rotor_position: u8,
    /// right rotor initial position, default is 0
    #[structopt(default_value = "0")]
    right_rotor_position: u8,
}

impl Cli {
    fn read_input(&self) -> Vec<u8> {
        match &self.input {
            None => KeyedReader::from(stdin()).collect(),
            Some(path) => {
                let file = File::open(path).expect("cannot open file");
                KeyedReader::from(file).collect()
            }
        }
    }

    fn create_engima(&self) -> EnigmaMachine {
        EnigmaMachine::new(
            self.reflector.into(),
            self.left_rotor.into(),
            self.middle_rotor.into(),
            self.right_rotor.into(),
            self.left_rotor_position,
            self.middle_rotor_position,
            self.right_rotor_position,
        )
        .unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();
    let mut machine = cli.create_engima();
    let input = cli.read_input();
    let enciphered = input.iter().map(|b| machine.encipher(*b)).collect();
    let output = String::from_utf8(enciphered).expect("cannot read bytes");
    print!("{}", output);
    Ok(())
}
