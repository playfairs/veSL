use crate::{error::Error, parser::PngDocument};

pub fn validate(input: &[u8]) -> Result<PngDocument, Error> {
    PngDocument::parse(input)
}

pub fn validate_type(kind: [u8; 4]) -> Result<(), Error> {
    if kind.iter().all(|byte| byte.is_ascii_alphabetic()) && kind[2].is_ascii_uppercase() {
        Ok(())
    } else {
        Err(Error::InvalidChunkType(kind))
    }
}
