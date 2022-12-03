use super::{Error, Parse, ParseExt};
use std::{collections::VecDeque, marker::PhantomData};

pub trait Separator {
    fn as_bytes() -> &'static [u8];
}

#[derive(Debug)]
struct CommaSep {}
impl Separator for CommaSep {
    fn as_bytes() -> &'static [u8] {
        ",".as_bytes()
    }
}

#[derive(Debug)]
pub struct EmptyLineSep {}
impl Separator for EmptyLineSep {
    fn as_bytes() -> &'static [u8] {
        "\n\n".as_bytes()
    }
}

#[derive(Debug)]
pub struct LineSep {}
impl Separator for LineSep {
    fn as_bytes() -> &'static [u8] {
        "\n".as_bytes()
    }
}
#[derive(Debug)]
pub struct Seq<T: Parse + Default, S: Separator> {
    ln: usize,
    p: PhantomData<T>,
    sep: PhantomData<S>,
    res: Vec<T::Out>,
    accepted: Vec<u8>,
    potential: VecDeque<u8>,
}

impl<T: Parse + Default, S: Separator> Default for Seq<T, S> {
    fn default() -> Self {
        Self {
            ln: 0,
            p: Default::default(),
            sep: Default::default(),
            res: Default::default(),
            accepted: Default::default(),
            potential: Default::default(),
        }
    }
}

impl<T: Parse + Default, S: Separator> Parse for Seq<T, S> {
    type Out = Vec<T::Out>;

    fn read_byte(&mut self, byte: &u8) -> Result<(), Error> {
        if self.potential.len() == S::as_bytes().len() {
            if self.potential.make_contiguous() == S::as_bytes() {
                let (ln_end, item) = T::parse_with_context(self.ln, &self.accepted).map_err(|(lne, e)| {
                    let mut elts = Vec::new();
                    elts.append(&mut self.res);
                    Error::new(e.context, e.message, lne)
                })?;
                self.ln = ln_end + S::as_bytes().iter().filter(|v| **v == 0xA).count();
                self.res.push(item);
                self.potential.clear();
                self.accepted.clear();
            } else if let Some(byte) = self.potential.pop_front() {
                self.accepted.push(byte);
            }
        }
        self.potential.push_back(*byte);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Out, Error> {
        if !self.potential.is_empty() && self.potential != S::as_bytes() {
            self.accepted.extend(self.potential.to_owned())
        }
        if !self.accepted.is_empty() {
            match T::parse(&self.accepted) {
                Ok(item) => self.res.push(item),
                Err(e) => return Err(Error::new(e.context, e.message, self.ln)),
            }
        }
        Ok(self.res)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::seq::LineSep;

    use super::{super::natural::Natural, super::ParseExt, CommaSep, Error, Seq};

    #[test]
    fn it_parses_a_vec_of_usize() {
        let bytes = "123".as_bytes();
        let numbers = Seq::<Natural<usize>, CommaSep>::parse(bytes).unwrap();
        assert_eq!(numbers, vec![123]);

        let bytes = "123,456,001,111,222".as_bytes();
        let numbers = Seq::<Natural<usize>, CommaSep>::parse(bytes).unwrap();
        assert_eq!(numbers, vec![123, 456, 1, 111, 222]);
    }

    #[test]
    fn it_parses_a_vec_of_vec_of_usize() -> Result<(), Error> {
        let bytes = "123,".as_bytes();
        let numbers = Seq::<Seq<Natural<usize>, CommaSep>, LineSep>::parse(bytes)?;
        assert_eq!(numbers, vec![vec![123]]);

        let bytes = r#"123,456
001,111
222"#
            .as_bytes();
        let numbers = Seq::<Seq<Natural<usize>, CommaSep>, LineSep>::parse(bytes).unwrap();
        assert_eq!(numbers, vec![vec![123, 456], vec![1, 111], vec![222]]);
        Ok(())
    }
}
