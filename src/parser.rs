use std::convert::From;

pub struct BufferParser {
    bytes: Box<std::vec::IntoIter<u8>>
}

impl From<Vec<u8>> for BufferParser {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes: Box::new(bytes.into_iter()) }
    }
}

impl Iterator for BufferParser {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        for byte in self.bytes.as_mut() {
            match byte {
                byte if byte.is_ascii_whitespace() => continue,
                byte if byte.is_ascii_punctuation() => continue,
                byte if byte.is_ascii_uppercase() => return Some(byte),
                byte if byte.is_ascii_lowercase() => return Some(byte - 0x20),
                _ => panic!("cannot represent byte '{}'", byte),
            };
        }
        None
    }
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;
//     use std::fs::File;
//     use std::io::{self, Write};

//     use tempfile::NamedTempFile;

//     use super::*;

//     #[test]
//     fn parser_yields_encoded_bytes() -> Result<(), Box<dyn Error>> {
//         let file = write_as_temp_file("Hello")?;

//         let mut parser = InputParser::from(file);

//         assert_eq!(b'H', parser.next().unwrap());
//         assert_eq!(b'E', parser.next().unwrap());

//         Ok(())
//     }

//     #[test]
//     fn parser_ignores_space_and_punctuation() -> Result<(), Box<dyn Error>> {
//         let file = write_as_temp_file("Hello, world!")?;

//         let parser = InputParser::from(file);
//         let vector: Vec<u8> = parser.collect();
//         assert_eq!(10, vector.len());

//         Ok(())
//     }

//     #[test]
//     fn parser_encodes_uppercase_and_lowercase_the_same() -> Result<(), Box<dyn Error>> {
//         let file1 = write_as_temp_file("Hello, world!")?;
//         let file2 = write_as_temp_file("HELLO, WORLD!!!!")?;

//         let input1: Vec<u8> = InputParser::from(file1).collect();
//         let input2: Vec<u8> = InputParser::from(file2).collect();

//         assert_eq!(input1, input2);
//         Ok(())
//     }

//     fn write_as_temp_file(content: &str) -> Result<File, io::Error> {
//         let mut file = NamedTempFile::new()?;
//         file.write_all(content.as_bytes())?;
//         file.reopen()
//     }
// }
