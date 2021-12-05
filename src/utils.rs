use std::io::{Bytes, Read};

pub fn parse_byte(byte: u8) -> Option<u8> {
    match byte {
        byte if byte.is_ascii_whitespace() => None,
        byte if byte.is_ascii_punctuation() => None,
        byte if byte.is_ascii_uppercase() => Some(byte),
        byte if byte.is_ascii_lowercase() => Some(byte.to_ascii_uppercase()),
        _ => panic!("cannot represent byte '{}'", byte),
    }
}

/// A wrapper for a sequences of bytes to treat as the "message".
///
/// By treating the
pub struct KeyedReader<T>
where
    T: Read,
{
    bytes: Box<Bytes<T>>,
}

impl<T: Read> From<T> for KeyedReader<T> {
    fn from(readable: T) -> Self {
        Self {
            bytes: Box::new(readable.bytes()),
        }
    }
}

impl<T> Iterator for KeyedReader<T>
where
    T: Read,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        for byte in self.bytes.as_mut() {
            match byte {
                Ok(byte) => match parse_byte(byte) {
                    Some(byte) => return Some(byte),
                    None => continue,
                },
                Err(err) => panic!("{}", err.to_string()),
            };
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, Write};

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn reader_yields_encoded_bytes() -> Result<(), Box<dyn Error>> {
        let file = write_as_temp_file("Hello")?;

        let mut reader = KeyedReader::from(file);

        assert_eq!(b'H', reader.next().unwrap());
        assert_eq!(b'E', reader.next().unwrap());

        Ok(())
    }

    #[test]
    fn reader_ignores_space_and_punctuation() -> Result<(), Box<dyn Error>> {
        let file = write_as_temp_file("Hello, world!")?;

        let reader = KeyedReader::from(file);
        let vector: Vec<u8> = reader.collect();
        assert_eq!(10, vector.len());

        Ok(())
    }

    #[test]
    fn reader_encodes_uppercase_and_lowercase_the_same() -> Result<(), Box<dyn Error>> {
        let file1 = write_as_temp_file("Hello, world!")?;
        let file2 = write_as_temp_file("HELLO, WORLD!!!!")?;

        let input1: Vec<u8> = KeyedReader::from(file1).collect();
        let input2: Vec<u8> = KeyedReader::from(file2).collect();

        assert_eq!(input1, input2);
        Ok(())
    }

    fn write_as_temp_file(content: &str) -> Result<File, io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write_all(content.as_bytes())?;
        file.reopen()
    }
}
