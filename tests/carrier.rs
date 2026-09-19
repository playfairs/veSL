use vesl::{PngChunk, embed_payload, extract_payload};

fn valid_png() -> Vec<u8> {
    let ihdr = [0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0];
    let mut png = Vec::new();
    png.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);
    png.extend_from_slice(
        &PngChunk::new(*b"IHDR", ihdr.to_vec())
            .unwrap()
            .encode()
            .unwrap(),
    );
    png.extend_from_slice(
        &PngChunk::new(*b"IDAT", vec![0; 16])
            .unwrap()
            .encode()
            .unwrap(),
    );
    png.extend_from_slice(
        &PngChunk::new(*b"IEND", Vec::new())
            .unwrap()
            .encode()
            .unwrap(),
    );
    png
}

#[test]
fn embeds_and_extracts_binary_payload() {
    let payload = [0u8, 1, 2, 3, 4, 255, 254, 253, 0, 7];
    let png = valid_png();
    let embedded = embed_payload(&png, &payload).unwrap();
    let extracted = extract_payload(&embedded).unwrap();
    assert_eq!(extracted, payload);
    assert!(embedded.starts_with(&[137, 80, 78, 71]));
}

#[test]
fn rejects_missing_payload() {
    let png = valid_png();
    let err = extract_payload(&png).unwrap_err();
    assert!(matches!(err, vesl::error::Error::MissingPayload));
}
