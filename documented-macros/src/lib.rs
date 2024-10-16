mod config;
mod derive_impl;
pub(crate) mod util;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Error, Ident, Item};

#[cfg(feature = "customise")]
use crate::config::attr::AttrCustomisations;
use crate::{
    config::attr::AttrConfig,
    derive_impl::{documented_fields_impl, documented_impl, documented_variants_impl, DocType},
    util::{get_docs, get_vis_name_attrs},
};

/// Derive proc-macro for `Documented` trait.
///
/// # Example
///
/// ```rust
/// use documented::Documented;
///
/// /// Nice.
/// /// Multiple single-line doc comments are supported.
/// ///
/// /** Multi-line doc comments are supported too.
///     Each line of the multi-line block is individually trimmed by default.
///     Note the lack of spaces in front of this line.
/// */
/// #[doc = "Attribute-style documentation is supported too."]
/// #[derive(Documented)]
/// struct BornIn69;
///
/// let doc_str = "Nice.
/// Multiple single-line doc comments are supported.
///
/// Multi-line doc comments are supported too.
/// Each line of the multi-line block is individually trimmed by default.
/// Note the lack of spaces in front of this line.
///
/// Attribute-style documentation is supported too.";
/// assert_eq!(BornIn69::DOCS, doc_str);
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented(...)]` attribute.
///
/// Currently, you can disable line-trimming like so:
///
/// ```rust
/// # use documented::Documented;
/// ///     Terrible.
/// #[derive(Documented)]
/// #[documented(trim = false)]
/// struct Frankly;
///
/// assert_eq!(Frankly::DOCS, "     Terrible.");
/// ```
///
/// If there are other configuration options you wish to have, please submit an
/// issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(Documented))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(Documented, attributes(documented))
)]
pub fn documented(input: TokenStream) -> TokenStream {
    documented_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedOpt` trait.
///
/// See [`Documented`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedOpt, attributes(documented))
)]
pub fn documented_opt(input: TokenStream) -> TokenStream {
    documented_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedFields` trait.
///
/// # Example
///
/// ```rust
/// use documented::DocumentedFields;
///
/// #[derive(DocumentedFields)]
/// struct BornIn69 {
///     /// Cry like a grandmaster.
///     rawr: String,
///     /// Before what?
///     explosive: usize,
/// };
///
/// assert_eq!(
///     BornIn69::FIELD_DOCS,
///     ["Cry like a grandmaster.", "Before what?"]
/// );
/// ```
///
/// You can also use [`get_field_docs`](Self::get_field_docs) to access the
/// fields' documentation using their names.
///
/// ```rust
/// # use documented::{DocumentedFields, Error};
/// #
/// # #[derive(DocumentedFields)]
/// # struct BornIn69 {
/// #     /// Cry like a grandmaster.
/// #     rawr: String,
/// #     /// Before what?
/// #     explosive: usize,
/// # };
/// #
/// assert_eq!(
///     BornIn69::get_field_docs("rawr"),
///     Ok("Cry like a grandmaster.")
/// );
/// assert_eq!(BornIn69::get_field_docs("explosive"), Ok("Before what?"));
/// assert_eq!(
///     BornIn69::get_field_docs("gotcha"),
///     Err(Error::NoSuchField("gotcha".to_string()))
/// );
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented_fields(...)]` attribute. Note that this
/// attribute works on both the container and each individual field, with the
/// per-field configurations overriding container configurations, which
/// override the default.
///
/// Currently, you can (selectively) disable line-trimming like so:
///
/// ```rust
/// # use documented::DocumentedFields;
/// #[derive(DocumentedFields)]
/// #[documented_fields(trim = false)]
/// struct Frankly {
///     ///     Delicious.
///     perrier: usize,
///     ///     I'm vegan.
///     #[documented_fields(trim = true)]
///     fried_liver: bool,
/// }
///
/// assert_eq!(Frankly::FIELD_DOCS, ["     Delicious.", "I'm vegan."]);
/// ```
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedFields))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedFields, attributes(documented_fields))
)]
pub fn documented_fields(input: TokenStream) -> TokenStream {
    documented_fields_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedFieldsOpt` trait.
///
/// See [`DocumentedFields`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedFieldsOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedFieldsOpt, attributes(documented_fields))
)]
pub fn documented_fields_opt(input: TokenStream) -> TokenStream {
    documented_fields_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedVariants` trait.
///
/// # Example
///
/// ```rust
/// use documented::{DocumentedVariants, Error};
///
/// #[derive(DocumentedVariants)]
/// enum NeverPlay {
///     /// Terrible.
///     F3,
///     /// I fell out of my chair.
///     F6,
/// }
///
/// assert_eq!(NeverPlay::F3.get_variant_docs(), "Terrible.");
/// assert_eq!(NeverPlay::F6.get_variant_docs(), "I fell out of my chair.");
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented_variants(...)]` attribute. Note that this
/// attribute works on both the container and each individual variant, with the
/// per-variant configurations overriding container configurations, which
/// override the default.
///
/// Currently, you can (selectively) disable line-trimming like so:
///
/// ```rust
/// # use documented::DocumentedVariants;
/// #[derive(DocumentedVariants)]
/// #[documented_variants(trim = false)]
/// enum Always {
///     ///     Or the quality.
///     SacTheExchange,
///     ///     Like a Frenchman.
///     #[documented_variants(trim = true)]
///     Retreat,
/// }
/// assert_eq!(
///     Always::SacTheExchange.get_variant_docs(),
///     "     Or the quality."
/// );
/// assert_eq!(Always::Retreat.get_variant_docs(), "Like a Frenchman.");
/// ```
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedVariants))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedVariants, attributes(documented_variants))
)]
pub fn documented_variants(input: TokenStream) -> TokenStream {
    documented_variants_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedVariantsOpt` trait.
///
/// See [`DocumentedVariants`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedVariantsOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedVariantsOpt, attributes(documented_variants))
)]
pub fn documented_variants_opt(input: TokenStream) -> TokenStream {
    documented_variants_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Macro to extract the documentation on any item that accepts doc comments
/// and store it in a const variable.
///
/// By default, this const variable inherits visibility from its parent item.
/// This can be manually configured; see configuration section below.
///
/// # Examples
///
/// ```rust
/// use documented::docs_const;
///
/// /// This is a test function
/// #[docs_const]
/// fn test_fn() {}
///
/// assert_eq!(TEST_FN_DOCS, "This is a test function");
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using attribute arguments.
///
/// Currently, you can:
///
/// ## 1. set a custom constant visibility like so:
///
/// ```rust
/// mod submodule {
///     use documented::docs_const;
///     
///     /// Boo!
///     #[docs_const(vis = pub)]
///     struct Wooooo;
/// }
///
/// // notice how the constant can be seen from outside
/// assert_eq!(submodule::WOOOOO_DOCS, "Boo!");
/// ```
///
/// ## 2. set a custom constant name like so:
///
/// ```rust
/// use documented::docs_const;
///
/// /// If you have a question raise your hand
/// #[docs_const(name = "DONT_RAISE_YOUR_HAND")]
/// mod whatever {}
///
/// assert_eq!(DONT_RAISE_YOUR_HAND, "If you have a question raise your hand");
/// ```
///
/// ## 3. disable line-trimming like so:
///
/// ```rust
/// use documented::docs_const;
///
/// ///     This is a test constant
/// #[docs_const(trim = false)]
/// const test_const: u8 = 0;
///
/// assert_eq!(TEST_CONST_DOCS, "     This is a test constant");
/// ```
///
/// ---
///
/// Multiple option can be specified in a list like so:
/// `name = "FOO", trim = false`.
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[proc_macro_attribute]
pub fn docs_const(#[allow(unused_variables)] attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as Item);

    #[cfg(not(feature = "customise"))]
    let config = AttrConfig::default();
    #[cfg(feature = "customise")]
    let config = AttrConfig::default()
        .with_customisations(syn::parse_macro_input!(attr as AttrCustomisations));

    let (item_vis, item_name, attrs) = match get_vis_name_attrs(&item) {
        Ok(pair) => pair,
        Err(e) => return e.into_compile_error().into(),
    };

    let docs = match get_docs(attrs, config.trim) {
        Ok(Some(docs)) => docs,
        Ok(None) => {
            // IDEA: customisation: allow_empty
            return Error::new_spanned(&item, "Missing doc comments")
                .into_compile_error()
                .into();
        }
        Err(e) => return e.into_compile_error().into(),
    };

    let const_vis = config.custom_vis.unwrap_or(item_vis);
    let const_name = config
        .custom_name
        .unwrap_or_else(|| format!("{}_DOCS", item_name.to_case(Case::ScreamingSnake)));
    let const_ident = Ident::new(&const_name, Span::call_site());

    // insert a const after the docs
    quote! {
        #item
        #const_vis const #const_ident: &'static str = #docs;
    }
    .into()
}
