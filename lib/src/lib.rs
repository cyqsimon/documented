pub use documented_derive::{Documented, DocumentedFields, DocumentedVariants};

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
/// You can also use [`DocumentedFields::get_field_docs`] to access the
/// fields' documentation using their names.
///
/// For enums, you may find [`DocumentedVariants`] more ergonomic to use.
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
    /// method will always return [`Error::NoSuchField`].
    fn get_field_docs<T: AsRef<str>>(field_name: T) -> Result<&'static str, Error> {
        let field_name = field_name.as_ref();
        let index = Self::__documented_get_index(field_name)
            .ok_or_else(|| Error::NoSuchField(field_name.into()))?;
        Self::FIELD_DOCS[index].ok_or_else(|| Error::NoDocComments(field_name.into()))
    }

    /// Deprecated alias for [`DocumentedFields::get_field_docs`].
    #[deprecated(
        since = "0.3.0",
        note = "This function has an inconsistent name. Use `DocumentedFields::get_field_docs` instead."
    )]
    #[inline]
    fn get_field_comment<T: AsRef<str>>(field_name: T) -> Result<&'static str, Error> {
        Self::get_field_docs(field_name)
    }
}

/// Adds an associated function [`DocumentedVariants::get_variant_docs`] to access
/// the documentation on an enum variant.
pub trait DocumentedVariants {
    /// Get the documentation on this enum variant.
    fn get_variant_docs(&self) -> Result<&'static str, Error>;
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
