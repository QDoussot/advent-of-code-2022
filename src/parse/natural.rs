use super::Context;
use super::Error;
use super::Parse;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

#[derive(Debug)]
pub struct Natural<T: FromStr> {
    p: PhantomData<T>,
    bytes: Vec<u8>,
}
impl<T: FromStr> Default for Natural<T> {
    fn default() -> Self {
        Self {
            p: Default::default(),
            bytes: vec![],
        }
    }
    //
}

impl<T, E: Display> Parse for Natural<T>
where
    T: FromStr<Err = E>,
{
    type Out = T;

    fn read_byte(&mut self, byte: &u8, _context: Context) -> Result<(), Error> {
        self.bytes.push(*byte);
        Ok(())
    }

    fn end(self, context: Context) -> Result<Self::Out, Error> {
        let string = std::str::from_utf8(&self.bytes).map_err(|_| Error::new("utf8 shit", "", 0))?;
        T::from_str(string).map_err(|e| Error::new(string.to_string(), e.to_string(), context.line))
    }
}

#[cfg(test)]
mod tests {

    use super::super::ParseExt;
    use super::Natural;

    #[test]
    fn it_parse_a_usize() {
        let bytes = "43945".as_bytes();
        let int = Natural::<usize>::parse(bytes);
        assert_eq!(int, Ok(43945));
    }

    #[test]
    fn it_fails_parsing_wrong_value() {
        let bytes = "".as_bytes();
        let int = Natural::<usize>::parse(bytes);
        assert!(int.is_err());
    }
}
