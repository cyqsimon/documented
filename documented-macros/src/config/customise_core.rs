use itertools::Itertools;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Expr, LitBool, LitStr, Meta, Token, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(vis);
    custom_keyword!(rename);
    custom_keyword!(default);
    custom_keyword!(trim);

    // recognised old keywords
    // error when used
    custom_keyword!(name);
}

/// A configuration option that includes the span info. Each kind of
/// customisation struct may choose to accept or reject any of them.
///
/// Expected parse stream format: `<KW> = <VAL>`.
#[derive(Clone, Debug)]
pub struct ConfigOption {
    /// The span over the keyword of the config option.
    pub span: Span,

    /// The config key-value pair.
    pub data: ConfigOptionData,
}
impl Parse for ConfigOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use ConfigOptionData as Data;
        use ConfigOptionKind as Kind;

        let span = input.span();

        let kind = input.parse::<ConfigOptionKind>()?;
        input.parse::<Token![=]>()?;
        let data = match kind {
            Kind::Vis => Data::Vis(input.parse()?),
            Kind::Rename => Data::Rename(input.parse()?),
            Kind::Default => Data::Default(input.parse()?),
            Kind::Trim => Data::Trim(input.parse()?),
        };

        Ok(Self { span, data })
    }
}

/// The data of all known configuration options.
#[derive(Clone, Debug, PartialEq, Eq, strum::EnumDiscriminants)]
#[strum_discriminants(
    vis(pub(self)),
    name(ConfigOptionKind),
    derive(strum::Display, Hash),
    strum(serialize_all = "snake_case")
)]
pub enum ConfigOptionData {
    /// Custom visibility for the generated constant.
    ///
    /// E.g. `vis = pub(crate)`.
    Vis(Visibility),

    /// Custom name for generated constant.
    ///
    /// E.g. `rename = "CUSTOM_NAME_DOCS"`.
    Rename(LitStr),

    /// Use some default value when doc comments are absent.
    ///
    /// E.g. `default = "not documented"`.
    Default(Expr),

    /// Trim each line or not.
    ///
    /// E.g. `trim = false`.
    Trim(LitBool),
}

impl Parse for ConfigOptionKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if lookahead.peek(kw::vis) {
            input.parse::<kw::vis>()?;
            Self::Vis
        } else if lookahead.peek(kw::rename) {
            input.parse::<kw::rename>()?;
            Self::Rename
        } else if lookahead.peek(kw::default) {
            input.parse::<kw::default>()?;
            Self::Default
        } else if lookahead.peek(kw::trim) {
            input.parse::<kw::trim>()?;
            Self::Trim
        } else if lookahead.peek(kw::name) {
            Err(Error::new(
                input.span(),
                "`name` has been removed; use `rename` instead",
            ))?
        } else {
            Err(lookahead.error())?
        };
        Ok(ty)
    }
}

/// Make sure there are no duplicate options.
/// Otherwise produces an error with detailed span info.
pub fn ensure_unique_options(opts: &[ConfigOption]) -> syn::Result<()> {
    for (kind, opts) in opts
        .iter()
        .into_group_map_by(|opt| ConfigOptionKind::from(&opt.data))
        .into_iter()
    {
        match &opts[..] {
            [] => unreachable!(), // guaranteed by `into_group_map_by`
            [_unique] => continue,
            [first, rest @ ..] => {
                let initial_error = Error::new(
                    first.span,
                    format!("Option {kind} can only be declaration once"),
                );
                let final_error = rest.iter().fold(initial_error, |mut err, opt| {
                    err.combine(Error::new(opt.span, "Duplicate declaration here"));
                    err
                });
                Err(final_error)?
            }
        }
    }
    Ok(())
}

/// Parse a list of attributes into a validated customisation.
///
/// `impl TryFrom<Vec<ConfigOption>>` and using this function is preferred to
/// `impl syn::parse::Parse` directly for situations where the options can come
/// from multiple attributes and therefore multiple `MetaList`s.
pub fn get_customisations_from_attrs<T>(attrs: &[Attribute], attr_name: &str) -> syn::Result<T>
where
    T: TryFrom<Vec<ConfigOption>, Error = syn::Error>,
{
    let options = attrs
        .iter()
        // remove irrelevant attributes
        .filter(|attr| attr.path().is_ident(attr_name))
        // parse options
        .map(|attr| match &attr.meta {
            Meta::List(attr_inner) => {
                attr_inner.parse_args_with(Punctuated::<ConfigOption, Token![,]>::parse_terminated)
            }
            other_form => Err(syn::Error::new(
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
