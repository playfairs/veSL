pub mod chunk;
pub mod error;
pub mod parser;
pub mod validator;

pub use chunk::PngChunk;
pub use parser::PngDocument;

pub const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
pub const VESSEL_TYPE: [u8; 4] = *b"veSL";

pub fn embed_payload(image: &[u8], payload: &[u8]) -> Result<Vec<u8>, error::Error> {
    let mut document = validator::validate(image)?;
    if document
        .chunks
        .iter()
        .any(|chunk| chunk.kind == VESSEL_TYPE)
    {
        return Err(error::Error::MultipleVesselChunks);
    }
    let vessel = PngChunk::new(VESSEL_TYPE, payload.to_vec())?;
    let iend = document
        .chunks
        .iter()
        .position(|chunk| chunk.kind == *b"IEND")
        .ok_or(error::Error::MissingIend)?;
    document.chunks.insert(iend, vessel);
    document.encode()
}

pub fn extract_payload(image: &[u8]) -> Result<Vec<u8>, error::Error> {
    let document = validator::validate(image)?;
    let vessels: Vec<_> = document
        .chunks
        .iter()
        .filter(|chunk| chunk.kind == VESSEL_TYPE)
        .collect();
    if vessels.len() > 1 {
        return Err(error::Error::MultipleVesselChunks);
    }
    let vessel = vessels.first().ok_or(error::Error::MissingPayload)?;
    Ok(vessel.data.clone())
}
