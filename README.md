# documented

Derive macro for accessing your type's documentation at runtime

## Quick start

```rust
/// Nice.
///
/// Multi-line doc comments are concatenated.
#[doc = "Attribute-style documentation is supported too."]
#[derive(documented::Documented)]
struct BornIn69;

let doc_str = "Nice.

Multi-line doc comments are concatenated.
Attribute-style documentation is supported too.";
assert_eq!(BornIn69::DOCS, doc_str);
```
