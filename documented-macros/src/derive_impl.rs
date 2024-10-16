//! Shared implementation for non-opt and opt variants of the derive macros.
//!
//! All functions in this module use the dependency injection pattern to
//! generate the correct trait implementation for both macro variants.

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, Ident,
};

#[cfg(feature = "customise")]
use crate::config::derive::get_customisations_from_attrs;
use crate::{
    config::derive::DeriveConfig,
    util::{crate_module_path, get_docs},
};

/// The type of the doc comment.
#[derive(Copy, Clone, Debug)]
pub enum DocType {
    /// &'static str
    Str,
    /// Option<&'static str>
    OptStr,
}
impl ToTokens for DocType {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let tokens = match self {
            Self::Str => quote! { &'static str },
            Self::OptStr => quote! { Option<&'static str> },
        };
        ts.append_all([tokens]);
    }
}
impl DocType {
    /// Get the closure that determines how to handle optional docs.
    /// The closure takes two arguments:
    ///
    /// 1. The optional doc comments on an item
    /// 2. The span on which to report any errors
    ///
    /// And fallibly returns the tokenised doc comments.
    fn docs_handler_opt(&self) -> Box<dyn Fn(Option<String>, Span) -> syn::Result<TokenStream>> {
        match self {
            Self::Str => Box::new(|docs_opt, span| match docs_opt {
                Some(docs) => Ok(quote! { #docs }),
                None => Err(Error::new(span, "Missing doc comments")),
            }),
            Self::OptStr => Box::new(|docs_opt, _span| {
                // quote macro needs some help with `Option`s
                // see: https://github.com/dtolnay/quote/issues/213
                let tokens = match docs_opt {
                    Some(docs) => quote! { Some(#docs) },
                    None => quote! { None },
                };
                Ok(tokens)
            }),
        }
    }

    /// Get the trait identifier, given a prefix.
    fn trait_ident_for(&self, prefix: &str) -> Ident {
        let name = match self {
            Self::Str => prefix.to_string(),
            Self::OptStr => format!("{prefix}Opt"),
        };
        Ident::new(&name, Span::call_site())
    }
}

/// Shared implementation of `Documented` & `DocumentedOpt`.
pub fn documented_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("Documented");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    #[cfg(not(feature = "customise"))]
    let config = DeriveConfig::default();
    #[cfg(feature = "customise")]
    let config = get_customisations_from_attrs(&input.attrs, "documented")
        .map(|c| DeriveConfig::default().with_customisations(c))?;

    let docs = get_docs(&input.attrs, config.trim)
        .and_then(|docs_opt| docs_ty.docs_handler_opt()(docs_opt, ident.span()))?;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            const DOCS: #docs_ty = #docs;
        }
    })
}

/// Shared implementation of `DocumentedFields` & `DocumentedFieldsOpt`.
pub fn documented_fields_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("DocumentedFields");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_fields(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = DeriveConfig::default();
    #[cfg(feature = "customise")]
    let base_config = get_customisations_from_attrs(&input.attrs, "documented_fields")
        .map(|c| DeriveConfig::default().with_customisations(c))?;

    let fields_attrs: Vec<_> = match input.data.clone() {
        Data::Enum(DataEnum { variants, .. }) => variants
            .into_iter()
            .map(|v| (v.span(), Some(v.ident), v.attrs))
            .collect(),
        Data::Struct(DataStruct { fields, .. }) => fields
            .into_iter()
            .map(|f| (f.span(), f.ident, f.attrs))
            .collect(),
        Data::Union(DataUnion { fields, .. }) => fields
            .named
            .into_iter()
            .map(|f| (f.span(), f.ident, f.attrs))
            .collect(),
    };

    let (field_idents, field_docs) = fields_attrs
        .into_iter()
        .map(|(span, ident, attrs)| {
            #[cfg(not(feature = "customise"))]
            let config = base_config;
            #[cfg(feature = "customise")]
            let config = base_config
                .with_customisations(get_customisations_from_attrs(&attrs, "documented_fields")?);
            get_docs(&attrs, config.trim)
                .and_then(|docs_opt| docs_ty.docs_handler_opt()(docs_opt, span))
                .map(|docs| (ident, docs))
        })
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let phf_match_arms = field_idents
        .into_iter()
        .enumerate()
        .filter_map(|(i, o)| o.map(|ident| (i, ident.to_string())))
        .map(|(i, ident)| quote! { #ident => #i, })
        .collect::<Vec<_>>();

    let documented_module_path = crate_module_path();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            const FIELD_DOCS: &'static [#docs_ty] = &[#(#field_docs),*];

            fn __documented_get_index<__Documented_T: AsRef<str>>(field_name: __Documented_T) -> Option<usize> {
                use #documented_module_path::_private_phf_reexport_for_macro as phf;

                static PHF: phf::Map<&'static str, usize> = phf::phf_map! {
                    #(#phf_match_arms)*
                };
                PHF.get(field_name.as_ref()).copied()
            }
        }
    })
}

/// Shared implementation of `DocumentedVariants` & `DocumentedVariantsOpt`.
pub fn documented_variants_impl(input: DeriveInput, docs_ty: DocType) -> syn::Result<TokenStream> {
    let trait_ident = docs_ty.trait_ident_for("DocumentedVariants");
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // `#[documented_variants(...)]` on container type
    #[cfg(not(feature = "customise"))]
    let base_config = DeriveConfig::default();
    #[cfg(feature = "customise")]
    let base_config = get_customisations_from_attrs(&input.attrs, "documented_variants")
        .map(|c| DeriveConfig::default().with_customisations(c))?;

    let Data::Enum(DataEnum { variants, .. }) = input.data else {
        Err(Error::new(
            input.span(), // this targets the `struct`/`union` keyword
            "DocumentedVariants can only be used on enums.\n\
            For structs and unions, use DocumentedFields instead.",
        ))?
    };

    let variants_docs = variants
        .into_iter()
        .map(|v| (v.span(), v.ident, v.fields, v.attrs))
        .map(|(span, ident, field, attrs)| {
            #[cfg(not(feature = "customise"))]
            let config = base_config;
            #[cfg(feature = "customise")]
            let config = base_config.with_customisations(get_customisations_from_attrs(
                &attrs,
                "documented_variants",
            )?);
            get_docs(&attrs, config.trim)
                .and_then(|docs_opt| docs_ty.docs_handler_opt()(docs_opt, span))
                .map(|docs| (ident, field, docs))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let match_arms = variants_docs
        .into_iter()
        .map(|(ident, fields, docs)| {
            let pat = match fields {
                Fields::Unit => quote! { Self::#ident },
                Fields::Unnamed(_) => quote! { Self::#ident(..) },
                Fields::Named(_) => quote! { Self::#ident{..} },
            };
            quote! { #pat => #docs, }
        })
        .collect::<Vec<_>>();

    // IDEA: I'd like to use phf here, but it doesn't seem to be possible at the moment,
    // because there isn't a way to get an enum's discriminant at compile time
    // if this becomes possible in the future, or alternatively you have a good workaround,
    // improvement suggestions are more than welcomed
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics documented::#trait_ident for #ident #ty_generics #where_clause {
            fn get_variant_docs(&self) -> #docs_ty {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })
}
