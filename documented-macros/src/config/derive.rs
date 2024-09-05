#[cfg(feature = "customise")]
use optfield::optfield;
#[cfg(feature = "customise")]
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Meta, Token,
};

#[cfg(feature = "customise")]
use crate::config::customise_core::ConfigOption;

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

#[cfg(feature = "customise")]
impl Parse for DeriveCustomisations {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use ConfigOption as O;

        let args = Punctuated::<ConfigOption, Token![,]>::parse_terminated(input)?;

        let mut config = Self::default();
        for arg in args {
            match arg {
                O::Vis(..) | O::Name(..) => Err(Error::new(
                    arg.kw_span(),
                    "This config option is not applicable to derive macros",
                ))?,

                O::Trim(..) if config.trim.is_some() => Err(Error::new(
                    arg.kw_span(),
                    "This config option cannot be specified more than once",
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
) -> syn::Result<Option<DeriveCustomisations>> {
    let customise_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident(attr_name))
        .map(|attr| match &attr.meta {
            Meta::List(attr_inner) => Ok(attr_inner),
            other_form => Err(Error::new(
                other_form.span(),
                format!("{attr_name} is not list-like. Expecting `{attr_name}(...)`"),
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let customise_attr = match customise_attrs.len() {
        0 => return Ok(None),
        1 => customise_attrs[0].clone(),
        _ => {
            let mut it = customise_attrs.iter();
            let initial_error = Error::new(
                it.next().unwrap().span(),
                format!("{attr_name} can only be declared once"),
            );
            let final_error = it.fold(initial_error, |mut err, declaration| {
                err.combine(Error::new(declaration.span(), "Duplicate declaration here"));
                err
            });
            Err(final_error)?
        }
    };

    let customisations = parse2(customise_attr.tokens)?;
    Ok(Some(customisations))
}
