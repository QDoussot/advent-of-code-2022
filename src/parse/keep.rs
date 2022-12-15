use super::{Context, Error, Parse};

#[derive(Default)]
pub struct Keep {
    bytes: Vec<u8>,
    start_context: Option<Context>,
}

impl Parse for Keep {
    type Out = (Vec<u8>, Context, Context);

    fn read_byte(&mut self, byte: &u8, context: Context) -> Result<(), Error> {
        if self.start_context.is_none() {
            self.start_context = Some(context);
        }
        self.bytes.push(*byte);
        Ok(())
    }

    fn end(self, context: Context) -> Result<Self::Out, Error> {
        Ok((self.bytes, self.start_context.unwrap_or(context), context))
    }
}
