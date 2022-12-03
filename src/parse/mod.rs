trait Parse {
    type Out;
    type Error;
    fn read_byte(&mut self, byte: &u8) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Out, Self::Error>;
}

trait ParseExt: Parse + Default {
    fn parse(bytes: &[u8]) -> Result<Self::Out, Self::Error> {
        let mut parser = Self::default();
        for b in bytes {
            parser.read_byte(b)?;
        }
        parser.end()
    }
}
impl<T: Parse + Default> ParseExt for T {}

pub mod natural;
pub mod seq;
