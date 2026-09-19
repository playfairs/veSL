use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("PNG signature is invalid")]
    InvalidSignature,
    #[error("PNG is truncated while reading {context}")]
    Truncated { context: &'static str },
    #[error("PNG chunk type is invalid: {0:?}")]
    InvalidChunkType([u8; 4]),
    #[error("PNG chunk {kind} has length {length} exceeding the input")]
    ChunkLength { kind: String, length: u32 },
    #[error("PNG chunk {kind} has an invalid CRC")]
    ChunkCrc { kind: String },
    #[error("PNG is missing IHDR")]
    MissingIhdr,
    #[error("PNG is missing IEND")]
    MissingIend,
    #[error("PNG contains data after IEND")]
    DataAfterIend,
    #[error("PNG contains an invalid chunk order: {0}")]
    InvalidStructure(String),
    #[error("PNG contains multiple veSL chunks")]
    MultipleVesselChunks,
    #[error("Vessel payload is missing")]
    MissingPayload,
    #[error("Vessel payload is malformed: {0}")]
    MalformedPayload(String),
    #[error("embedded payload format error: {0}")]
    Format(String),
    #[error("image format is invalid: {0}")]
    InvalidImage(String),
    #[error("I/O error for {path}: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
}

pub fn io_error(path: &std::path::Path, source: std::io::Error) -> Error {
    Error::Io {
        path: path.display().to_string(),
        source,
    }
}
