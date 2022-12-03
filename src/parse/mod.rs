use derive_more::Display;

pub trait Parse {
    type Out;
    fn read_byte(&mut self, byte: &u8) -> Result<(), Error>;
    fn end(self) -> Result<Self::Out, Error>;
}

#[derive(Debug, Display, PartialEq, Eq)]
#[display(fmt = "{} {}", context, message)]
pub struct Error {
    //    current_line: String,
    context: String,
    message: String,
    line_number: usize,
}

impl Error {
    pub fn new(context: impl Into<String>, message: impl Into<String>, line_number: usize) -> Self {
        Self {
            context: context.into(),
            message: message.into(),
            line_number,
        }
    }
}

pub trait ParseExt: Parse + Default {
    fn parse(bytes: &[u8]) -> Result<Self::Out, Error> {
        let mut parser = Self::default();
        for b in bytes {
            parser.read_byte(b)?;
        }
        parser.end()
    }

    fn parse_with_context(mut line_number: usize, bytes: &[u8]) -> Result<(usize, Self::Out), (usize, Error)> {
        let mut parser = Self::default();
        //let mut line_number = 0;
        let mut current_line = vec![];
        for b in bytes {
            if b == &0xA {
                line_number += 1;
                current_line.clear()
            } else {
                current_line.push(*b);
            }
            parser.read_byte(b).map_err(|e| (line_number, e))?;
        }
        let out = parser.end().map_err(|e| (line_number, e));
        out.map(|out| (line_number, out))
    }
}
impl<T: Parse + Default> ParseExt for T {}

pub mod natural;
pub mod seq;
