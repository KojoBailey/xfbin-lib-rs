# Rust XFBIN Library
This is a Rust library for deserialising (reading) and serialising (writing) [CyberConnect2](https://jojomodding.miraheze.org/wiki/CyberConnect2) **[XFBIN](https://jojomodding.miraheze.org/wiki/XFBIN)** files.

XFBIN files have a file signature of `NUCC` (hex: `4E 55 43 43`), which this library relies on to identify them (as opposed to the `.xfbin` file extension).
