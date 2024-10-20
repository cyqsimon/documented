use syn::Expr;

/// Configurable options for derive macros via helper attributes.
///
/// Initial values are set to default.
#[cfg_attr(feature = "customise", optfield::optfield(
    pub DeriveCustomisations,
    attrs = add(derive(Default)),
    merge_fn = pub apply_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeriveConfig {
    // optfield does not rewrap `Option` by default, which is the desired behavior
    // see https://docs.rs/optfield/latest/optfield/#rewrapping-option-fields
    pub default_value: Option<Expr>,
    pub trim: bool,
}
impl Default for DeriveConfig {
    fn default() -> Self {
        Self { default_value: None, trim: true }
    }
}

#[cfg(feature = "customise")]
pub mod customise {
    use syn::{punctuated::Punctuated, spanned::Spanned, Attribute, Error, Meta, Token};

    use crate::config::{
        customise_core::{ensure_unique_options, ConfigOption, ConfigOptionData},
        derive::{DeriveConfig, DeriveCustomisations},
    };

    impl DeriveConfig {
        /// Return a new instance of this config with customisations applied.
        pub fn with_customisations(&self, customisations: DeriveCustomisations) -> Self {
            let mut new = self.clone();
            new.apply_customisations(customisations);
            new
        }
    }

    // This is implemented instead of `syn::parse::Parse` because the options
    // can come from multiple attributes and therefore multiple `MetaList`s.
    impl TryFrom<Vec<ConfigOption>> for DeriveCustomisations {
        type Error = syn::Error;

        /// Duplicate option rejection should be handled upstream.
        fn try_from(opts: Vec<ConfigOption>) -> Result<Self, Self::Error> {
            use ConfigOptionData as Data;

            let mut config = Self::default();
            for opt in opts {
                match opt.data {
                    Data::Vis(..) | Data::Rename(..) => Err(Error::new(
                        opt.span,
                        "This config option is not applicable to derive macros",
                    ))?,
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
                Meta::List(attr_inner) => attr_inner
                    .parse_args_with(Punctuated::<ConfigOption, Token![,]>::parse_terminated),
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
}
