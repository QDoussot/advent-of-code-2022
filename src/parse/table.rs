use super::{separator::Separator, Context, Error, Parse, ParseExt};
use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
enum Token {
    Item,
    Separator,
}
impl Default for Token {
    fn default() -> Self {
        Token::Item
    }
}

pub struct Table<const N: usize, S: Separator, T: Parse + Default> {
    p: PhantomData<T>,
    sep: PhantomData<S>,
    previous_context: Option<Context>,
    accepted: Vec<u8>,
    curr_token: Token,
    res: Vec<T::Out>,
}

impl<const N: usize, S: Separator, T: Parse + Default> Default for Table<N, S, T> {
    fn default() -> Self {
        Self {
            p: Default::default(),
            sep: Default::default(),
            previous_context: Default::default(),
            accepted: Default::default(),
            curr_token: Default::default(),
            res: Default::default(),
        }
    }
}

impl<const N: usize, S: Separator, T: Parse + Default> Parse for Table<N, S, T> {
    type Out = Vec<T::Out>;

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        let previous_context = self.previous_context.unwrap_or(context);
        self.previous_context = Some(previous_context);
        match self.curr_token {
            Token::Item => {
                self.accepted.push(*byte);
                if self.accepted.len() == N {
                    let item = T::parse_with_context(&self.accepted, previous_context)?;
                    self.res.push(item);
                    if S::as_bytes().len() > 0 {
                        self.curr_token = Token::Separator;
                        self.previous_context = None;
                    }

                    self.accepted.clear();
                }
                Ok(())
            }
            Token::Separator => {
                self.accepted.push(*byte);
                if self.accepted.len() == S::as_bytes().len() {
                    if self.accepted == S::as_bytes() {
                        self.curr_token = Token::Item;
                        self.accepted.clear();
                        self.previous_context = None;
                        Ok(())
                    } else {
                        Err(Error::new(
                            std::str::from_utf8(&self.accepted).unwrap(),
                            format!("Wrong separator, expected '{:?}'", S::as_bytes()),
                            previous_context.line,
                        ))
                    }
                } else {
                    Ok(())
                }
            }
        }
    }

    fn end(self, context: Context) -> Result<Self::Out, Error> {
        if self.curr_token == Token::Item && self.accepted.len() != S::as_bytes().len() {
            Err(Error::new(
                std::str::from_utf8(&self.accepted).unwrap(),
                format!("Too small item in table"),
                context.line,
            ))
        } else {
            Ok(self.res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{natural::Natural, separator::StrSep, table::Table, StaticStr};



    #[test]
    fn it_parse_vec_of_usize() {
        struct Space{}
        impl StaticStr for Space {
            fn as_str() -> &'static str {
                " "
            }
        }
        let bytes = "LOL KIK --- LOL KIK".as_bytes();
        type Parser = Table<3, StrSep<Space>, Natural<String>>;
        let res = Parser::parse(bytes).unwrap();
        println!("{res:?}",);
    }
}
