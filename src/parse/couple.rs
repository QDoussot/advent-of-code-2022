use std::{collections::VecDeque, marker::PhantomData, str::from_utf8};

use super::{separator::Separator, Context, Error, Parse, ParseExt};

#[derive(PartialEq, Eq)]
pub enum SplitMode {
    Exact,
    First,
}

pub trait SplitModeLike {
    fn split_mode() -> SplitMode;
}

pub struct Exact {}
impl SplitModeLike for Exact {
    fn split_mode() -> SplitMode {
        SplitMode::Exact
    }
}

pub struct SplitFirst {}
impl SplitModeLike for SplitFirst {
    fn split_mode() -> SplitMode {
        SplitMode::First
    }
}

#[derive(Debug)]
pub struct Couple<T1: Parse + Default, S: Separator, T2: Parse + Default, SM: SplitModeLike = Exact> {
    previous_context: Context,
    p1: PhantomData<T1>,
    p2: PhantomData<T2>,
    sep: PhantomData<S>,
    split_mode: PhantomData<SM>,
    res: (Option<T1::Out>, Option<T2::Out>),
    accepted: Vec<u8>,
    potential: VecDeque<u8>,
}

impl<T1: Parse + Default, T2: Parse + Default, S: Separator, SM: SplitModeLike> Default for Couple<T1, S, T2, SM> {
    fn default() -> Self {
        Self {
            previous_context: Default::default(),
            p1: Default::default(),
            p2: Default::default(),
            sep: Default::default(),
            split_mode: Default::default(),
            res: Default::default(),
            accepted: Default::default(),
            potential: Default::default(),
        }
    }
}

impl<T1: Parse + Default, T2: Parse + Default, S: Separator, SM: SplitModeLike> Parse for Couple<T1, S, T2, SM>
where
    T1::Out: std::fmt::Debug,
    T2::Out: std::fmt::Debug,
{
    type Out = (T1::Out, T2::Out);

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        self.potential.push_back(*byte);
        if self.potential.len() == S::as_bytes().len() {
            if self.potential.make_contiguous() == S::as_bytes() {
                if self.res.0.is_none() {
                    let item = T1::parse_with_context(&self.accepted, self.previous_context)?;
                    self.res.0 = Some(item);
                    self.potential.clear();
                    self.accepted.clear();
                } else if SM::split_mode() == SplitMode::Exact {
                    return Err(Error::new("More than one field", "", context.line));
                }
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
        if !self.potential.is_empty() {
            if self.potential == S::as_bytes() && SM::split_mode() == SplitMode::Exact {
                return Err(Error::new(
                    "",
                    format!("Final unexpected separator: '{:?}'", S::as_bytes()),
                    0,
                ));
            } else {
                self.accepted.extend(self.potential.to_owned())
            }
        }
        if self.res.0.is_none() {
            return Err(Error::new(
                from_utf8(&self.accepted).unwrap(),
                format!("Reached end without finding separator: '{:?}'", S::as_bytes()),
                context.line,
            ));
        }
        match T2::parse_with_context(&self.accepted, context) {
            Ok(item) => self.res.1 = Some(item),
            Err(err) => return Err(err),
        }
        Ok((self.res.0.unwrap(), self.res.1.unwrap()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parse::{natural::Natural, separator::SpaceSep};

    #[test]
    fn it_parse_a_couple() {
        let couple = "200 Coucou".as_bytes();
        let couple = Couple::<Natural<usize>, SpaceSep, Natural<String>>::parse(couple).unwrap();
        assert_eq!((200, "Coucou".to_string()), couple);
    }

    #[test]
    fn it_separates_at_first_separator_when_specified() {
        let couple = "200 Coucou les loulous".as_bytes();
        let couple = Couple::<Natural<usize>, SpaceSep, Natural<String>, SplitFirst>::parse(couple).unwrap();
        assert_eq!((200, "Coucou les loulous".to_string()), couple);
    }
}
