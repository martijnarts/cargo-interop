# Cargo Interop

Make it very easy to communicate between Rust and any other language, as natively as possible.

Install with `cargo install cargo-interop`, and then run `cargo interop setup` for setup instructions.

## Long version

1. Add your target language configuration to your Cargo.toml:

   ```toml
   [interop.java]
   minimum_version = 11
   build_system = gradle
   ```

   (see [supported languages](languages/), each has a README to check out)

2. Run `cargo interop build` somewhere in your build system to ensure your Rust code gets built and linked correctly.

## How?

Cargo Interop uses code generation and serde's Serializable and Deserializable traits and type system to allow near-zero interop between languages. To make it easy to add new languages and maintain a base set of features for all languages, we pass data back and forth as CBOR by default. Better supported languages support sending native Rust types directly between functions.
