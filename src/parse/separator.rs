pub trait Separator {
    fn as_bytes() -> &'static [u8];
}

#[derive(Debug)]
pub struct CommaSep {}
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
pub struct SpaceSep {}
impl Separator for SpaceSep {
    fn as_bytes() -> &'static [u8] {
        " ".as_bytes()
    }
}

#[derive(Debug)]
pub struct StrSep<const S: &'static str> {}

impl<const S: &'static str> Separator for StrSep<S> {
    fn as_bytes() -> &'static [u8] {
        S.as_bytes()
    }
}
