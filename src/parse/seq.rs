use super::{separator::Separator, Context, Error, Parse, ParseExt};
use std::{collections::VecDeque, marker::PhantomData};

#[derive(PartialEq, Eq)]
pub enum EmptyBehavior {
    Skip,
    Keep,
    SkipFinal,
}

pub trait EmptyBehaviorLike {
    fn behavior() -> EmptyBehavior;
}

pub struct Keep {}
impl EmptyBehaviorLike for Keep {
    fn behavior() -> EmptyBehavior {
        EmptyBehavior::Keep
    }
}

pub struct Skip {}
impl EmptyBehaviorLike for Skip {
    fn behavior() -> EmptyBehavior {
        EmptyBehavior::Skip
    }
}

pub struct SkipFinal {}
impl EmptyBehaviorLike for SkipFinal {
    fn behavior() -> EmptyBehavior {
        EmptyBehavior::SkipFinal
    }
}

#[derive(Debug)]
pub struct Seq<T: Parse + Default, S: Separator, EB: EmptyBehaviorLike = Keep> {
    previous_context: Context,
    p: PhantomData<T>,
    sep: PhantomData<S>,
    empty_beh: PhantomData<EB>,
    res: Vec<T::Out>,
    accepted: Vec<u8>,
    potential: VecDeque<u8>,
}

impl<T: Parse + Default, S: Separator, EB: EmptyBehaviorLike> Default for Seq<T, S, EB> {
    fn default() -> Self {
        Self {
            previous_context: Default::default(),
            p: Default::default(),
            sep: Default::default(),
            empty_beh: Default::default(),
            res: Default::default(),
            accepted: Default::default(),
            potential: Default::default(),
        }
    }
}

impl<T: Parse + Default, S: Separator, EB: EmptyBehaviorLike> Parse for Seq<T, S, EB> {
    type Out = Vec<T::Out>;

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        self.potential.push_back(*byte);

        if self.potential.len() == S::as_bytes().len() {
            if self.potential.make_contiguous() == S::as_bytes() {
                if !self.accepted.is_empty() || EB::behavior() != EmptyBehavior::Skip {
                    let item = T::parse_with_context(&self.accepted, self.previous_context)?;
                    self.res.push(item);
                }
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

    fn end(mut self, _: Context) -> Result<Self::Out, Error> {
        self.accepted.extend(self.potential.to_owned());
        if !self.accepted.is_empty() || EB::behavior() == EmptyBehavior::Keep {
            let item = T::parse_with_context(&self.accepted, self.previous_context)?;
            self.res.push(item);
        }
        Ok(self.res)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{
        separator::{CommaSep, LineSep},
        seq::Skip,
    };

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

    #[test]
    fn it_skips_empty_field_if_specified() -> Result<(), Error> {
        let bytes = ",lol,,coucou,,".as_bytes();
        let expected = Seq::<Natural<String>, CommaSep, Skip>::parse(bytes)?;
        assert_eq!(
            expected.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
            vec!["lol", "coucou",]
        );
        Ok(())
    }

    #[test]
    fn it_skips_empty_field_if_specified_2() -> Result<(), Error> {
        let bytes = ",".as_bytes();
        let expected = Seq::<Natural<String>, CommaSep, Skip>::parse(bytes)?;
        assert_eq!(
            expected.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
            Vec::<&str>::new()
        );
        Ok(())
    }
}
