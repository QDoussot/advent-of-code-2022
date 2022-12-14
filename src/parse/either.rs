use std::{marker::PhantomData, str::from_utf8};

use super::{Context, Error, Parse, ParseExt};

#[derive(Default)]
pub struct Either<P1: Parse + Default, P2: Parse + Default> {
    p1: PhantomData<P1>,
    p2: PhantomData<P2>,
    buffer: Vec<u8>,
    start_context: Option<Context>,
}

impl<P1: Parse + Default, P2: Parse + Default> Parse for Either<P1, P2> {
    type Out = either::Either<P1::Out, P2::Out>;

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        self.buffer.push(*byte);
        if self.start_context.is_none() {
            self.start_context = Some(context);
        }
        Ok(())
    }

    fn end(self, context: Context) -> Result<Self::Out, Error> {
        if let Ok(p1_out) = P1::parse(&self.buffer) {
            Ok(either::Either::Left(p1_out))
        } else if let Ok(p2_out) = P2::parse(&self.buffer) {
            Ok(either::Either::Right(p2_out))
        } else {
            Err(Error {
                context: from_utf8(&self.buffer).unwrap().to_string(),
                message: "Neither parser succeed".to_string(),
                line_number: context.line,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{couple::Couple, natural::Natural, separator::SpaceSep};

    use super::*;

    #[test]
    fn it_parses_either_couple_or_usize() {
        type Parser = Either<Natural<usize>, Couple<Natural<usize>, SpaceSep, Natural<usize>>>;
        let tests = [("1 2", either::Right((1, 2))), ("1", either::Left(1))];
        tests.into_iter().for_each(|(input, expected)| {
            assert_eq!(Parser::parse(input.as_bytes()), Ok(expected));
        });
    }
}
