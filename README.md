# documented

Derive macro for accessing your type's documentation at runtime

[crates.io](https://crates.io/crates/documented)

## Quick start

```rust
use documented::Documented;

fn main() {
    /// Nice.
    /// Multiple single-line doc comments are supported.
    ///
    /** Multi-line doc comments are supported too.
    Each line of the multi-line block is individually trimmed.
    Note the lack of spaces in front of this line.
    */
    #[doc = "Attribute-style documentation is supported too."]
    #[derive(Documented)]
    struct BornIn69;

    let doc_str = "Nice.
Multiple single-line doc comments are supported.

Multi-line doc comments are supported too.
Each line of the multi-line block is individually trimmed.
Note the lack of spaces in front of this line.

Attribute-style documentation is supported too.";
    assert_eq!(BornIn69::DOCS, doc_str);
}
```
