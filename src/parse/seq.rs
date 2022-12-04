use super::{separator::Separator, Context, Error, Parse, ParseExt};
use std::{collections::VecDeque, marker::PhantomData};

#[derive(Debug)]
pub struct Seq<T: Parse + Default, S: Separator> {
    previous_context: Context,
    p: PhantomData<T>,
    sep: PhantomData<S>,
    res: Vec<T::Out>,
    accepted: Vec<u8>,
    potential: VecDeque<u8>,
}

impl<T: Parse + Default, S: Separator> Default for Seq<T, S> {
    fn default() -> Self {
        Self {
            previous_context: Default::default(),
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

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        self.potential.push_back(*byte);

        if self.potential.len() == S::as_bytes().len() {
            if self.potential.make_contiguous() == S::as_bytes() {
                let item = T::parse_with_context(&self.accepted, self.previous_context)?;
                self.res.push(item);
                self.potential.clear();
                self.accepted.clear();
            } else if let Some(byte) = self.potential.pop_front() {
                if self.accepted.is_empty() {
                    self.previous_context = context
                }
                self.accepted.push(byte);
            }
        }
        Ok(())
    }

    fn end(mut self, context: Context) -> Result<Self::Out, Error> {
        self.accepted.extend(self.potential.to_owned());
        match T::parse_with_context(&self.accepted, context) {
            Ok(item) => self.res.push(item),
            Err(err) => return Err(err),
        }
        Ok(self.res)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::separator::{CommaSep, LineSep};

    use super::{super::natural::Natural, super::ParseExt, Error, Seq};

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
        let bytes = "123".as_bytes();
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

    #[test]
    fn it_parses_a_vec_of_string() -> Result<(), Error> {
        let bytes = ",,coucou,,".as_bytes();
        let expected = Seq::<Natural<String>, CommaSep>::parse(bytes)?;
        assert_eq!(
            expected.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
            vec!["", "", "coucou", "", ""]
        );
        Ok(())
    }
}
