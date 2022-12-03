use derive_more::Display;

use super::Parse;
use std::{
    fmt::Display,
    marker::PhantomData,
    str::{FromStr, Utf8Error},
};

#[derive(Default, Debug)]
pub struct Natural<T: FromStr> {
    p: PhantomData<T>,
    bytes: Vec<u8>,
}

#[derive(Eq, PartialEq, Debug, Display)]
pub enum NaturalError<FE> {
    FromUtf8(Utf8Error),
    FromStr(FE),
}
impl<T, E: Display> Parse for Natural<T>
where
    T: FromStr<Err = E>,
{
    type Out = T;
    type Error = NaturalError<E>;

    fn read_byte(&mut self, byte: &u8) -> Result<(), Self::Error> {
        self.bytes.push(*byte);
        Ok(())
    }

    fn end(self) -> Result<Self::Out, Self::Error> {
        let string = std::str::from_utf8(&self.bytes).map_err(Self::Error::FromUtf8)?;
        T::from_str(string).map_err(Self::Error::FromStr)
    }
}

#[cfg(test)]
mod tests {

    use std::num::IntErrorKind;

    use super::super::ParseExt;
    use super::{Natural, NaturalError};

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
        match int.expect_err("An error is expected") {
            NaturalError::FromUtf8(_) => panic!(),
            NaturalError::FromStr(int_error) => assert_eq!(int_error.kind(), &IntErrorKind::Empty),
        }
    }
}
