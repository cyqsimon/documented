pub use documented_derive::{Documented, DocumentedFields};

#[doc(hidden)]
pub use phf as _private_phf_reexport_for_macro;

/// Adds an associated constant `DOCS` on your type containing its documentation,
/// allowing you to access its documentation at runtime.
pub trait Documented {
    /// The static doc comments on this type.
    const DOCS: &'static str;
}

pub trait DocumentedFields {
    /// The static doc comments on each field or variant of this type, indexed by
    /// field/variant order.
    const FIELD_DOCS: &'static [&'static str];

    fn get_index_by_name<T: AsRef<str>>(field_name: T) -> Option<usize>;

    fn get_field_comment<T: AsRef<str>>(field_name: T) -> Option<&'static str> {
        Self::get_index_by_name(field_name).map(|i| Self::FIELD_DOCS[i])
    }
}
