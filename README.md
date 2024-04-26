# filterable-enum

Filterable wrapper for discriminated unions(DU) in rust.

## What is this?

In some cases, you may want to filter a DU by a bitmask to check if it matches a subset of all variants.
This crate provides a derive macro to generate a filterable wrapper for a DU.

Ideally, I think this whole crate should be unnecessary. Rust already stores a tag for representing the variant of a DU.
However, Rust does not expose this tag to the user, so I cannot really use that tag as a bitflag.

## Note

This is the first procedural macro I've ever written, so I'm not sure if I'm doing it idiomatic or right.
Feel free to open an issue or PR if you have any suggestions or improvements.
