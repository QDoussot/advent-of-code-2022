use derive_more::Display;

#[derive(Debug, Copy, Clone)]
pub struct Context {
    line: usize,
    col: usize,
}
impl Default for Context {
    fn default() -> Self {
        Self { line: 1, col: 0 }
    }
}

pub trait Parse {
    type Out;
    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error>;
    fn end(self, context: Context) -> Result<Self::Out, Error>;
}

pub trait StaticStr {
    fn as_str() -> &'static str;
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
        Self::parse_with_context(bytes, Context::default())
    }

    fn parse_with_context(bytes: &[u8], mut context: Context) -> Result<Self::Out, Error> {
        let mut parser = Self::default();
        //let mut line_number = 0;
        let mut current_line = vec![];
        for b in bytes {
            if b == &0xA {
                context.line += 1;
                context.col = 0;
                current_line.clear()
            } else {
                context.col += 1;
                current_line.push(*b);
            }
            parser.read_byte(b, context)?;
        }
        parser.end(context)
    }
}
impl<T: Parse + Default> ParseExt for T {}

pub mod capture;
pub mod couple;
pub mod either;
pub mod keep;
pub mod natural;
pub mod separator;
pub mod seq;
pub mod table;
