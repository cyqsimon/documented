use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    DataUnion, DeriveInput, Error, Expr, ExprLit, Fields, Lit, Meta, Path,
};

fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

/// Derive proc-macro for `Documented` trait.
#[proc_macro_derive(Documented)]
pub fn documented(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let doc_comments = match get_comment(&input.attrs) {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return Error::new(input.ident.span(), "Missing doc comments")
                .into_compile_error()
                .into()
        }
        Err(e) => return e.into_compile_error().into(),
    };

    let ident = input.ident;

    quote! {
        #[automatically_derived]
        impl documented::Documented for #ident {
            const DOCS: &'static str = #doc_comments;
        }
    }
    .into()
}

/// Derive proc-macro for `DocumentedFields` trait.
#[proc_macro_derive(DocumentedFields)]
pub fn documented_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields_doc_comments = {
        let fields_attrs: Vec<(Option<syn::Ident>, Vec<Attribute>)> = match input.data.clone() {
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
            .map(|(i, attrs)| get_comment(&attrs).map(|c| (i, c)))
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(t) => t,
            Err(e) => return e.into_compile_error().into(),
        }
    };

    let ident = input.ident;

    let (field_idents, field_comments): (Vec<_>, Vec<_>) = fields_doc_comments.into_iter().unzip();

    // quote macro needs some help with `Option`s
    // see: https://github.com/dtolnay/quote/issues/213
    let field_comments_tokenised: Vec<_> = field_comments
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
        impl documented::DocumentedFields for #ident {
            const FIELD_DOCS: &'static [Option<&'static str>] = &[#(#field_comments_tokenised),*];

            fn __documented_get_index<T: AsRef<str>>(field_name: T) -> Option<usize> {
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
#[proc_macro_derive(DocumentedVariants)]
pub fn documented_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants_doc_comments = {
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
            .map(|(i, f, attrs)| get_comment(&attrs).map(|c| (i, f, c)))
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(t) => t,
            Err(e) => return e.into_compile_error().into(),
        }
    };

    let ident = input.ident;

    let match_arms: Vec<_> = variants_doc_comments
        .into_iter()
        .map(|(ident, fields, comment)| {
            let pat = match fields {
                Fields::Unit => quote! { Self::#ident },
                Fields::Unnamed(_) => quote! { Self::#ident(..) },
                Fields::Named(_) => quote! { Self::#ident{..} },
            };
            match comment {
                Some(comment) => quote! { #pat => Ok(#comment), },
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
        impl documented::DocumentedVariants for #ident {
            fn get_variant_docs(&self) -> Result<&'static str, documented::Error> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
    .into()
}

fn get_comment(attrs: &[Attribute]) -> syn::Result<Option<String>> {
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

    let trimmed: Vec<_> = string_literals
        .iter()
        .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
        .map(|line| line.trim().to_string())
        .collect();

    Ok(Some(trimmed.join("\n")))
}
