#[cfg(feature = "customise")]
use optfield::optfield;
#[cfg(feature = "customise")]
use syn::{punctuated::Punctuated, spanned::Spanned, Attribute, Error, Meta, Token};

#[cfg(feature = "customise")]
use crate::config::customise_core::{ensure_unique_options, ConfigOption};

/// Configurable options for derive macros via helper attributes.
///
/// Initial values are set to default.
#[cfg_attr(feature = "customise", optfield(
    pub DeriveCustomisations,
    attrs = add(derive(Default)),
    merge_fn = pub apply_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DeriveConfig {
    pub trim: bool,
}
impl Default for DeriveConfig {
    fn default() -> Self {
        Self { trim: true }
    }
}
#[cfg(feature = "customise")]
impl DeriveConfig {
    /// Return a new instance of this config with customisations applied.
    pub fn with_customisations(mut self, customisations: DeriveCustomisations) -> Self {
        self.apply_customisations(customisations);
        self
    }
}

// This is implemented instead of `syn::parse::Parse` because the options
// can come from multiple attributes and therefore multiple `MetaList`s.
#[cfg(feature = "customise")]
impl TryFrom<Vec<ConfigOption>> for DeriveCustomisations {
    type Error = syn::Error;

    /// Duplicate option rejection should be handled upstream.
    fn try_from(args: Vec<ConfigOption>) -> Result<Self, Self::Error> {
        use ConfigOption as O;

        let mut config = Self::default();
        for arg in args {
            match arg {
                O::Vis(..) | O::Name(..) => Err(Error::new(
                    arg.kw_span(),
                    "This config option is not applicable to derive macros",
                ))?,
                O::Trim(_, val) => {
                    config.trim.replace(val);
                }
            }
        }
        Ok(config)
    }
}

#[cfg(feature = "customise")]
pub fn get_customisations_from_attrs(
    attrs: &[Attribute],
    attr_name: &str,
) -> syn::Result<DeriveCustomisations> {
    let options = attrs
        .iter()
        // remove irrelevant attributes
        .filter(|attr| attr.path().is_ident(attr_name))
        // parse options
        .map(|attr| match &attr.meta {
            Meta::List(attr_inner) => {
                attr_inner.parse_args_with(Punctuated::<ConfigOption, Token![,]>::parse_terminated)
            }
            other_form => Err(Error::new(
                other_form.span(),
                format!("{attr_name} is not list-like. Expecting `{attr_name}(...)`"),
            )),
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    ensure_unique_options(&options)?;

    options.try_into()
}
