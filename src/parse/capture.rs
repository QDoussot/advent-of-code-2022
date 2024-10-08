use super::{Context, Error, Parse, ParseExt, StaticStr};

#[derive(PartialEq, Eq)]
enum PatternPart {
    Const,
    PlaceHolder,
    End,
}

use PatternPart::*;

pub struct Capture<S: StaticStr, const N: usize, T: Parse + Default> {
    s: std::marker::PhantomData<S>,
    capture_pos: usize,
    previous_context: Context,
    accepted: Vec<u8>,
    res: Vec<T::Out>,
}

impl<S: StaticStr, const N: usize, T: Parse + Default> Default for Capture<S, N, T> {
    fn default() -> Self {
        Self {
            s:Default::default(),
            capture_pos: Default::default(),
            previous_context: Default::default(),
            accepted: Default::default(),
            res: Default::default(),
        }
    }
}

impl<S: StaticStr, const N: usize, T: Parse + Default> Capture<S, N, T> {
    fn pattern_part(&self) -> PatternPart {
        if self.capture_pos < S::as_str().len() {
            match S::as_str().as_bytes()[self.capture_pos] {
                b'%' => PlaceHolder,
                _ => Const,
            }
        } else {
            End
        }
    }
}

impl<S: StaticStr, const N: usize, T: Parse + Default> Parse for Capture<S, N, T> {
    type Out = [T::Out; N];

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        match self.pattern_part() {
            PatternPart::PlaceHolder => {
                if self.capture_pos + 1 <= S::as_str().len() - 1 && S::as_str().as_bytes()[self.capture_pos + 1] == *byte {
                    self.capture_pos += 2;
                    let item = T::parse_with_context(&self.accepted, self.previous_context)?;
                    self.res.push(item);
                    self.accepted.clear();
                } else {
                    self.accepted.push(*byte);
                }
                Ok(())
            }
            PatternPart::Const => {
                let expected = S::as_str().as_bytes()[self.capture_pos];
                if expected != *byte {
                    Err(Error::new(
                        format!("{:?}", *byte),
                        format!("Expected {} => {:?}", self.capture_pos, expected),
                        context.line,
                    ))
                } else {
                    self.capture_pos += 1;
                    if self.capture_pos == S::as_str().len() {
                    } else if S::as_str().as_bytes()[self.capture_pos] == b'%' {
                        self.previous_context = context;
                    }
                    Ok(())
                }
            }
            End => Err(Error::new("", "Reached end of pattern", context.line)),
        }
    }

    fn end(mut self, context: Context) -> Result<Self::Out, Error> {
        match self.pattern_part() {
            Const => Err(Error::new("", "Premature end of input", context.line)),
            PlaceHolder if self.capture_pos == S::as_str().len() - 1 => {
                let item = T::parse_with_context(&self.accepted, self.previous_context)?;
                self.res.push(item);
                Ok(self.res)
            }
            PlaceHolder => Err(Error::new("", "Premature end of input", context.line)),
            End => Ok(self.res),
        }
        .and_then(|res| {
            res.try_into()
                .map_err(|_| Error::new("", "Not expected number of captured string", context.line))
        })
    }
    //
}

#[cfg(test)]
mod tests {
    use crate::parse::natural::Natural;

    use super::*;

    struct MoveFromTo {}
    impl StaticStr for MoveFromTo {
        fn as_str() -> &'static str {
            "move % from % to %"
        }
    }

    #[test]
    fn it_parses_the_move_command() {

        type Parser = Capture<MoveFromTo, 3, Natural<usize>>;
        let bytes = "move 32 from 101 to 202".as_bytes();
        let vec = Parser::parse(bytes).unwrap();
        assert_eq!(vec![32, 101, 202], vec);
    }

    struct Percent {}
    impl StaticStr for Percent {
        fn as_str() -> &'static str {
            "%"
        }
    }

    #[test]
    fn it_parses_single_usize() {
        type Parser = Capture<Percent, 1, Natural<usize>>;
        let bytes = "43".as_bytes();
        let vec = Parser::parse(bytes).unwrap();
        assert_eq!(vec![43], vec);
    }
}
