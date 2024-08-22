#[cfg(feature = "customise")]
use optfield::optfield;
#[cfg(feature = "customise")]
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, LitBool, Meta, Token,
};

/// Configurable options via helper attributes.
///
/// Initial values are set to default.
#[cfg_attr(feature = "customise", optfield(
    pub ConfigCustomisations,
    attrs = add(derive(Default)),
    merge_fn = pub apply_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub trim: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self { trim: true }
    }
}
impl Config {
    /// Return a new instance of this config with customisations applied.
    pub fn with_customisations(mut self, customisations: ConfigCustomisations) -> Self {
        self.apply_customisations(customisations);
        self
    }
}

#[cfg(feature = "customise")]
impl Parse for ConfigCustomisations {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<ConfigOption, Token![,]>::parse_terminated(input)?;

        let mut config = Self::default();
        for arg in args {
            match arg {
                ConfigOption::Trim(kw, _) if config.trim.is_some() => Err(Error::new(
                    kw.span(),
                    "This config option cannot be specified more than once",
                ))?,
                ConfigOption::Trim(_, val) => {
                    config.trim.replace(val);
                }
            }
        }
        Ok(config)
    }
}

#[cfg(feature = "customise")]
mod kw {
    use syn::custom_keyword;

    custom_keyword!(trim);
}

/// All known configuration options.
///
/// Expected parse stream format: `<KW> = <VAL>`.
#[cfg(feature = "customise")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ConfigOption {
    /// Trim each line or not.
    ///
    /// E.g. `trim = false`.
    Trim(kw::trim, bool),
}
#[cfg(feature = "customise")]
impl Parse for ConfigOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::trim) {
            let kw = input.parse::<kw::trim>()?;
            input.parse::<Token![=]>()?;
            let trim = input.parse::<LitBool>()?;
            Ok(Self::Trim(kw, trim.value))
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(feature = "customise")]
pub fn get_config_customisations(
    attrs: &[Attribute],
    attr_name: &str,
) -> syn::Result<Option<ConfigCustomisations>> {
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

    let customisations = parse2::<ConfigCustomisations>(customise_attr.tokens)?;
    Ok(Some(customisations))
}
