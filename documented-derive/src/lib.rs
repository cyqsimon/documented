mod config;

use config::ConfigCustomisations;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    DataUnion, DeriveInput, Error, Expr, ExprLit, Fields, Ident, Lit, Meta, Path,
};

#[cfg(feature = "customise")]
use crate::config::get_config_customisations;
use crate::config::Config;

fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

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
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    #[cfg(not(feature = "customise"))]
    let config = Config::default();
    #[cfg(feature = "customise")]
    let config = match get_config_customisations(&input.attrs, "documented") {
        Ok(Some(customisations)) => Config::default().with_customisations(customisations),
        Ok(None) => Config::default(),
        Err(err) => return err.into_compile_error().into(),
    };

    let docs = match get_docs(&input.attrs, &config) {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return Error::new(input.ident.span(), "Missing doc comments")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics documented::Documented for #ident #ty_generics #where_clause {
            const DOCS: &'static str = #docs;
        }
    }
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
///     explosive: usize,
/// };
///
/// assert_eq!(BornIn69::FIELD_DOCS, [Some("Cry like a grandmaster."), None]);
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
/// #     explosive: usize,
/// # };
/// #
/// assert_eq!(BornIn69::get_field_docs("rawr"), Ok("Cry like a grandmaster."));
/// assert_eq!(
///     BornIn69::get_field_docs("explosive"),
///     Err(Error::NoDocComments("explosive".to_string()))
/// );
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
/// assert_eq!(Frankly::FIELD_DOCS, [Some("     Delicious."), Some("I'm vegan.")]);
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
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_fields(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = Config::default();
    #[cfg(feature = "customise")]
    let base_config = match get_config_customisations(&input.attrs, "documented_fields") {
        Ok(Some(customisations)) => Config::default().with_customisations(customisations),
        Ok(None) => Config::default(),
        Err(err) => return err.into_compile_error().into(),
    };

    let (field_idents, field_docs): (Vec<_>, Vec<_>) = {
        let fields_attrs: Vec<(Option<Ident>, Vec<Attribute>)> = match input.data.clone() {
            Data::Enum(DataEnum { variants, .. }) => variants
                .into_iter()
                .map(|v| (Some(v.ident), v.attrs))
                .collect(),
            Data::Struct(DataStruct { fields, .. }) => {
                fields.into_iter().map(|f| (f.ident, f.attrs)).collect()
            }
            Data::Union(DataUnion { fields, .. }) => fields
                .named
                .into_iter()
                .map(|f| (f.ident, f.attrs))
                .collect(),
        };

        match fields_attrs
            .into_iter()
            .map(|(i, attrs)| {
                #[cfg(not(feature = "customise"))]
                let config = base_config;
                #[cfg(feature = "customise")]
                let config =
                    if let Some(c) = get_config_customisations(&attrs, "documented_fields")? {
                        base_config.with_customisations(c)
                    } else {
                        base_config
                    };
                get_docs(&attrs, &config).map(|d| (i, d))
            })
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(t) => t.into_iter().unzip(),
            Err(e) => return e.into_compile_error().into(),
        }
    };

    // quote macro needs some help with `Option`s
    // see: https://github.com/dtolnay/quote/issues/213
    let field_docs_tokenised: Vec<_> = field_docs
        .into_iter()
        .map(|opt| match opt {
            Some(c) => quote! { Some(#c) },
            None => quote! { None },
        })
        .collect();

    let phf_match_arms: Vec<_> = field_idents
        .into_iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|ident| (i, ident.to_string())))
        .map(|(i, ident)| quote! { #ident => #i, })
        .collect();

    let documented_module_path = crate_module_path();

    quote! {
        #[automatically_derived]
        impl #impl_generics documented::DocumentedFields for #ident #ty_generics #where_clause {
            const FIELD_DOCS: &'static [Option<&'static str>] = &[#(#field_docs_tokenised),*];

            fn __documented_get_index<__Documented_T: AsRef<str>>(field_name: __Documented_T) -> Option<usize> {
                use #documented_module_path::_private_phf_reexport_for_macro as phf;

                static PHF: phf::Map<&'static str, usize> = phf::phf_map! {
                    #(#phf_match_arms)*
                };
                PHF.get(field_name.as_ref()).copied()
            }
        }
    }
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
///     F3,
///     /// I fell out of my chair.
///     F6,
/// }
///
/// assert_eq!(
///     NeverPlay::F3.get_variant_docs(),
///     Err(Error::NoDocComments("F3".into()))
/// );
/// assert_eq!(
///     NeverPlay::F6.get_variant_docs(),
///     Ok("I fell out of my chair.")
/// );
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
///     Ok("     Or the quality.")
/// );
/// assert_eq!(Always::Retreat.get_variant_docs(), Ok("Like a Frenchman."));
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
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_variants(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = Config::default();
    #[cfg(feature = "customise")]
    let base_config = match get_config_customisations(&input.attrs, "documented_variants") {
        Ok(Some(customisations)) => Config::default().with_customisations(customisations),
        Ok(None) => Config::default(),
        Err(err) => return err.into_compile_error().into(),
    };

    let variants_docs = {
        let Data::Enum(DataEnum { variants, .. }) = input.data else {
            return Error::new(
                input.span(), // this targets the `struct`/`union` keyword
                "DocumentedVariants can only be used on enums.\n\
                For structs and unions, use DocumentedFields instead.",
            )
            .into_compile_error()
            .into();
        };
        match variants
            .into_iter()
            .map(|v| (v.ident, v.fields, v.attrs))
            .map(|(i, f, attrs)| {
                #[cfg(not(feature = "customise"))]
                let config = base_config;
                #[cfg(feature = "customise")]
                let config =
                    if let Some(c) = get_config_customisations(&attrs, "documented_variants")? {
                        base_config.with_customisations(c)
                    } else {
                        base_config
                    };
                get_docs(&attrs, &config).map(|d| (i, f, d))
            })
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(t) => t,
            Err(e) => return e.into_compile_error().into(),
        }
    };

    let match_arms: Vec<_> = variants_docs
        .into_iter()
        .map(|(ident, fields, docs)| {
            let pat = match fields {
                Fields::Unit => quote! { Self::#ident },
                Fields::Unnamed(_) => quote! { Self::#ident(..) },
                Fields::Named(_) => quote! { Self::#ident{..} },
            };
            match docs {
                Some(docs_str) => quote! { #pat => Ok(#docs_str), },
                None => {
                    let ident_str = ident.to_string();
                    quote! { #pat => Err(documented::Error::NoDocComments(#ident_str.into())), }
                }
            }
        })
        .collect();

    // IDEA: I'd like to use phf here, but it doesn't seem to be possible at the moment,
    // because there isn't a way to get an enum's discriminant at compile time
    // if this becomes possible in the future, or alternatively you have a good workaround,
    // improvement suggestions are more than welcomed
    quote! {
        #[automatically_derived]
        impl #impl_generics documented::DocumentedVariants for #ident #ty_generics #where_clause {
            fn get_variant_docs(&self) -> Result<&'static str, documented::Error> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
    .into()
}

fn get_docs(attrs: &[Attribute], config: &Config) -> syn::Result<Option<String>> {
    let string_literals = attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            Meta::NameValue(ref name_value) if name_value.path.is_ident("doc") => {
                Some(&name_value.value)
            }
            _ => None,
        })
        .map(|expr| match expr {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
            other => Err(Error::new(
                other.span(),
                "Doc comment is not a string literal",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if string_literals.is_empty() {
        return Ok(None);
    }

    let docs = if config.trim {
        string_literals
            .iter()
            .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        string_literals.join("\n")
    };

    Ok(Some(docs))
}

/// Also adds a macro to extract the docs from a function.
/// WIP!
#[proc_macro_attribute]
pub fn documented_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    // We ignore the attribute for now, but we should probably warn the user that it's not used

    // The item is a function, so we parse it as such
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    #[cfg(not(feature = "customise"))]
    let base_config = Config::default();
    #[cfg(feature = "customise")]
    let base_config = Config::default()
        .with_customisations(syn::parse_macro_input!(attr as ConfigCustomisations));

    // The docstring should be part of the function's attributes
    let attrs = &item.attrs;

    // We get the docs from the attributes
    let docs = match get_docs(attrs, &base_config) {
        Ok(Some(docs)) => docs,
        Ok(None) => {
            return Error::new(item.sig.span(), "Missing doc comments")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };

    // Now we want to keep the function the way it is, but also, after the function,
    // insert a const variable with the docs

    let doc_var_name = format!("{}_DOCS", item.sig.ident.to_string().to_uppercase()); // converts case to SCREAMING_SNAKE_CASE

    let doc_var_ident = Ident::new(&doc_var_name, item.sig.ident.span());

    let doc_var_vis = &item.vis;

    let doc_var = quote! {
        #[allow(non_upper_case_globals)]
        #doc_var_vis const #doc_var_ident: &'static str = #docs;
    };

    // We return the function as is, but with the docs variable added
    let function = quote! {
        #item
        #doc_var
    };

    function.into()
}
