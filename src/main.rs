use std::collections::VecDeque;
use std::io::{Read, Write};

use structopt::StructOpt;

use enigma::BufferParser;
use enigma::Cli;
use enigma::Reflector;
use enigma::RotorBank;


const LINE_LENGTH: usize = 80;

pub struct Enigma {
    reader: Box<dyn Read>,
    writer: Box<dyn Write>,
    reflector: Reflector,
    rotors: RotorBank,
}

impl Enigma {
    pub fn write_cipher(&mut self) {
        let failure_msg = "failed to write cipher to file";
        let buffer = self.read_input();
        let mut buffer = self.encipher_input(buffer);
        while buffer.len() > LINE_LENGTH {
            let buf: Vec<u8> = buffer.drain(0..LINE_LENGTH).collect();
            self.writer.write_all(&buf).expect(failure_msg);
            writeln!(self.writer).expect(failure_msg);
        }
        let buf: Vec<u8> = buffer.into();
        self.writer.write_all(&buf).expect(failure_msg);
    }

    fn read_input(&mut self) -> Vec<u8> {
        let failure_msg = "error reading file";
        let mut buf: Vec<u8> = Vec::new();
        self.reader.read_to_end(&mut buf).expect(failure_msg);
        BufferParser::from(buf).collect()
    }

    fn encipher_input(&mut self, input: Vec<u8>) -> VecDeque<u8> {
        let mut deque = VecDeque::new();
        let encipher = |byte: u8| self.encipher_byte(byte);
        let cipher = BufferParser::from(input).map(encipher);
        deque.extend(cipher);
        deque
    }

    fn encipher_byte(&mut self, byte: u8) -> u8 {
        let mut index = byte - 0x41;
        index = self.rotors.encipher_right_to_left(index);
        index = self.reflector.encipher(index);
        index = self.rotors.encipher_left_to_right(index);
        self.rotors.turn();
        index + 0x41
    }
}

impl From<Cli> for Enigma {
    fn from(cli: Cli) -> Self {
        let reader = cli.get_reader();
        let writer = cli.get_writer();
        let reflector = cli.reflector.into();
        let mut rotors = RotorBank {
            left: cli.left_rotor.into(),
            right: cli.right_rotor.into(),
            middle: cli.middle_rotor.into(),
        };
        rotors.set_positions(
            cli.left_rotor_position as u8 - 0x41,
            cli.right_rotor_position as u8 - 0x41, 
            cli.middle_rotor_position as u8 - 0x41,
        );
        Enigma { reader, writer, reflector, rotors }
    }
}

fn main() {
    let cli = Cli::from_args();
    cli.validate_args();
    let mut enigma = Enigma::from(cli);
    enigma.write_cipher();
}
