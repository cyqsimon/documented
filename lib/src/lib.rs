pub use documented_derive::{Documented, DocumentedFields};

#[doc(hidden)]
pub use phf as _private_phf_reexport_for_macro;

/// Adds an associated constant `DOCS` on your type containing its documentation,
/// allowing you to access its documentation at runtime.
pub trait Documented {
    /// The static doc comments on this type.
    const DOCS: &'static str;
}

/// Adds an associated constant `FIELD_DOCS` on your type containing the
/// documentation of its fields, allowing you to access their documentation at
/// runtime.
///
/// You can also use [`DocumentedFields::get_field_comment`] to access the
/// fields' documentation using their names.
pub trait DocumentedFields {
    /// The static doc comments on each field or variant of this type, indexed by
    /// field/variant order.
    const FIELD_DOCS: &'static [Option<&'static str>];

    /// Method internally used by `documented`.
    #[doc(hidden)]
    fn __documented_get_index<T: AsRef<str>>(field_name: T) -> Option<usize>;

    /// Get a field's documentation using its name.
    ///
    /// Note that for structs with anonymous fields (i.e. tuple structs), this
    /// method will be an empty stub and therefore not usable.
    fn get_field_comment<T: AsRef<str>>(field_name: T) -> Result<&'static str, Error> {
        let field_name = field_name.as_ref();
        let index = Self::__documented_get_index(field_name)
            .ok_or_else(|| Error::NoSuchField(field_name.into()))?;
        Self::FIELD_DOCS[index].ok_or_else(|| Error::NoDocComments(field_name.into()))
    }
}

/// Errors of `documented`.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// The requested field does not have doc comments.
    #[error(r#"The field "{0}" has no doc comments"#)]
    NoDocComments(String),
    /// The requested field does not exist.
    #[error(r#"No field named "{0}" exists"#)]
    NoSuchField(String),
}
