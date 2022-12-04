use std::{collections::VecDeque, marker::PhantomData};

use super::{separator::Separator, Context, Error, Parse, ParseExt};

#[derive(Debug)]
pub struct Couple<T1: Parse + Default, S: Separator, T2: Parse + Default> {
    previous_context: Context,
    p1: PhantomData<T1>,
    p2: PhantomData<T2>,
    sep: PhantomData<S>,
    res: (Option<T1::Out>, Option<T2::Out>),
    accepted: Vec<u8>,
    potential: VecDeque<u8>,
}

impl<T1: Parse + Default, T2: Parse + Default, S: Separator> Default for Couple<T1, S, T2> {
    fn default() -> Self {
        Self {
            previous_context: Default::default(),
            p1: Default::default(),
            p2: Default::default(),
            sep: Default::default(),
            res: Default::default(),
            accepted: Default::default(),
            potential: Default::default(),
        }
    }
}

impl<T1: Parse + Default, T2: Parse + Default, S: Separator> Parse for Couple<T1, S, T2> {
    type Out = (T1::Out, T2::Out);

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        if self.potential.len() == S::as_bytes().len() {
            if self.potential.make_contiguous() == S::as_bytes() {
                if self.res.0.is_none() {
                    let item = T1::parse_with_context(&self.accepted, self.previous_context)?;
                    self.res.0 = Some(item);
                    self.potential.clear();
                    self.accepted.clear();
                } else {
                    return Err(Error::new("More than one field", "", context.line));
                }
            } else if let Some(byte) = self.potential.pop_front() {
                if self.accepted.is_empty() {
                    self.previous_context = context
                }
                self.accepted.push(byte);
            }
        }
        self.potential.push_back(*byte);
        Ok(())
    }

    fn end(mut self, context: Context) -> Result<Self::Out, Error> {
        if !self.potential.is_empty() {
            if self.potential == S::as_bytes() {
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
                "",
                format!("Reached end without finding separator: '{:?}'", S::as_bytes()),
                0,
            ));
        }
        if !self.accepted.is_empty() {
            match T2::parse_with_context(&self.accepted, context) {
                Ok(item) => self.res.1 = Some(item),
                Err(err) => return Err(err),
            }
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
}
