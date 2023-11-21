# documented

Derive macro for accessing your type's documentation at runtime

[crates.io](https://crates.io/crates/documented)

## Quick start

```rust
use documented::{Documented, DocumentedFields, Error};

/// Nice.
/// Multiple single-line doc comments are supported.
///
/** Multi-line doc comments are supported too.
Each line of the multi-line block is individually trimmed.
Note the lack of spaces in front of this line.
*/
#[doc = "Attribute-style documentation is supported too."]
#[derive(Documented, DocumentedFields)]
struct BornIn69 {
    /// Doc comments on fields (and enum variants) are supported too using
    /// the `DocumentedFields` derive macro.
    ///
    /// Frankly, delicious.
    rawr: String,

    explosive: usize,
};

let doc_str = "Nice.
Multiple single-line doc comments are supported.

Multi-line doc comments are supported too.
Each line of the multi-line block is individually trimmed.
Note the lack of spaces in front of this line.

Attribute-style documentation is supported too.";
assert_eq!(BornIn69::DOCS, doc_str);

let field_doc_str = "Doc comments on fields (and enum variants) are supported too using
the `DocumentedFields` derive macro.

Frankly, delicious.";
assert_eq!(BornIn69::FIELD_DOCS, [Some(field_doc_str), None]);
assert_eq!(BornIn69::get_field_comment("rawr"), Ok(field_doc_str));
assert_eq!(
    BornIn69::get_field_comment("explosive"),
    Err(Error::NoDocComments("explosive".to_string()))
);
assert_eq!(
    BornIn69::get_field_comment("gotcha"),
    Err(Error::NoSuchField("gotcha".to_string()))
);
```
