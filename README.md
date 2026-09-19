# veSL

veSL is the image-level carrier layer. It is responsible for embedding and extracting opaque binary payloads inside a valid PNG container.

The design boundary is intentionally narrow:

```text
Image
  └── veSL carrier
       └── payload bytes
```

This repository does not implement the Vessel CLI, and it does not define the VSL payload format itself. Instead, it focuses on the PNG container mechanics: validating the image, locating a dedicated `veSL` chunk, inserting payload data, and extracting the payload again without disturbing the rest of the PNG structure.

## Relationship to VSL

- veSL is the carrier mechanism.
- VSL is the binary payload format.
- Vessel is the higher-level tool that may orchestrate the use of these layers.

In other words, veSL is not VSL, and VSL is not a PNG format.

## Current scope

- PNG signature and structure validation
- PNG chunk parsing and CRC checks
- `veSL` chunk insertion before `IEND`
- payload extraction from valid PNGs
- image-level malformed-input handling

## Development

This project uses Nox as the primary workflow interface.

```bash
nox validate
nox build
nox test
nox check
nox task fmt-check
nox task clippy
```

---

This repository is intentionally a focused library for containerization of arbitrary payload bytes in PNG files. It is not a general-purpose CLI or a full payload serialization format.
