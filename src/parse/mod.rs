use std::str::from_utf8;

use derive_more::Display;

pub trait Parse {
    type Out;
    type Error;
    fn read_byte(&mut self, byte: &u8) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Out, Self::Error>;
}

#[derive(Debug, Display)]
#[display(fmt = "{}: '{}' -> {}", line_number, current_line, under)]
pub struct Error<P: Parse> {
    line_number: usize,
    current_line: String,
    under: P::Error,
}

impl<P: Parse> Error<P> {
    fn new(line_number: usize, current_line: String, under: P::Error) -> Self {
        Self {
            line_number,
            current_line,
            under,
        }
    }
}

pub trait ParseExt: Parse + Default {
    fn parse(bytes: &[u8]) -> Result<Self::Out, Self::Error> {
        let mut parser = Self::default();
        for b in bytes {
            parser.read_byte(b)?;
        }
        parser.end()
    }

    fn parse_with_context(bytes: &[u8]) -> Result<Self::Out, Error<Self>> {
        let mut parser = Self::default();
        let mut line_number = 0;
        let mut current_line = vec![];
        for b in bytes {
            if b == &0xA {
                line_number += 1;
                current_line.clear()
            } else {
                current_line.push(*b);
            }
            parser
                .read_byte(b)
                .map_err(|e| Error::new(line_number, from_utf8(&current_line).unwrap().to_string(), e))?;
        }
        parser.end().map_err(|e| Error::new(line_number, String::new(), e))
    }
}
impl<T: Parse + Default> ParseExt for T {}

pub mod natural;
pub mod seq;
