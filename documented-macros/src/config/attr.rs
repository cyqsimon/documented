#[cfg(feature = "customise")]
use optfield::optfield;
#[cfg(feature = "customise")]
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};
use syn::{Expr, Visibility};

#[cfg(feature = "customise")]
use crate::config::customise_core::{ensure_unique_options, ConfigOption, ConfigOptionData};

/// Configurable arguments for attribute macros.
///
/// Initial values are set to default.
#[cfg_attr(feature = "customise", optfield(
    pub AttrCustomisations,
    attrs = add(derive(Default)),
    merge_fn = pub apply_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttrConfig {
    // optfield does not rewrap `Option` by default, which is the desired behavior
    // see https://docs.rs/optfield/latest/optfield/#rewrapping-option-fields
    pub custom_vis: Option<Visibility>,
    pub custom_name: Option<String>,
    pub default_value: Option<Expr>,
    pub trim: bool,
}
impl Default for AttrConfig {
    fn default() -> Self {
        Self {
            custom_vis: None,
            custom_name: None,
            default_value: None,
            trim: true,
        }
    }
}
#[cfg(feature = "customise")]
impl AttrConfig {
    /// Return a new instance of this config with customisations applied.
    pub fn with_customisations(mut self, customisations: AttrCustomisations) -> Self {
        self.apply_customisations(customisations);
        self
    }
}

#[cfg(feature = "customise")]
impl Parse for AttrCustomisations {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use ConfigOptionData as Data;

        let opts = Punctuated::<ConfigOption, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect::<Vec<_>>();

        ensure_unique_options(&opts)?;

        let mut config = Self::default();
        for opt in opts {
            // I'd love to macro this if declarative macros can expand to a full match arm,
            // but no: https://github.com/rust-lang/rfcs/issues/2654
            match opt.data {
                Data::Vis(vis) => {
                    config.custom_vis.replace(vis);
                }
                Data::Rename(name) => {
                    config.custom_name.replace(name.value());
                }
                Data::Default(expr) => {
                    config.default_value.replace(expr);
                }
                Data::Trim(trim) => {
                    config.trim.replace(trim.value());
                }
            }
        }
        Ok(config)
    }
}
