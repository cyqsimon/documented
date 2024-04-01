# documented

Derive macro for accessing your type's documentation at runtime

- [crates.io](https://crates.io/crates/documented)
- [docs.rs](https://docs.rs/documented/latest/documented/)

## Quick start

```rust
use documented::{Documented, DocumentedFields, DocumentedVariants, Error};

/// Trying is the first step to failure.
#[derive(Documented, DocumentedFields, DocumentedVariants)]
enum AlwaysPlay {
    Kb1,
    /// But only if you are white.
    F6,
}

// Documented
assert_eq!(AlwaysPlay::DOCS, "Trying is the first step to failure.");

// DocumentedFields
assert_eq!(
    AlwaysPlay::FIELD_DOCS,
    [None, Some("But only if you are white.")]
);
assert_eq!(
    AlwaysPlay::get_field_docs("Kb1"),
    Err(Error::NoDocComments("Kb1".to_string()))
);
assert_eq!(
    AlwaysPlay::get_field_docs("F6"),
    Ok("But only if you are white.")
);
assert_eq!(
    AlwaysPlay::get_field_docs("Bf1"),
    Err(Error::NoSuchField("Bf1".to_string()))
);

// DocumentedVariants
assert_eq!(
    AlwaysPlay::Kb1.get_variant_docs(),
    Err(Error::NoDocComments("Kb1".to_string()))
);
assert_eq!(
    AlwaysPlay::F6.get_variant_docs(),
    Ok("But only if you are white.")
);
```
