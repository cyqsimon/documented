pub use documented_derive::Documented;

/// Adds an associated constant `DOCS` on your type containing its documentation,
/// allowing you to access its documentation at runtime.
pub trait Documented {
    /// The static doc comments on this type.
    const DOCS: &'static str;
}
