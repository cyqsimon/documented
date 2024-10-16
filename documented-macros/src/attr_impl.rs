//! Implementation of the attribute macros.

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, Item};

#[cfg(feature = "customise")]
use crate::config::attr::AttrCustomisations;
use crate::{
    config::attr::AttrConfig,
    util::{get_docs, get_vis_name_attrs},
};

pub fn docs_const_impl(
    item: Item,
    #[cfg(feature = "customise")] customisations: AttrCustomisations,
) -> syn::Result<TokenStream> {
    #[cfg(not(feature = "customise"))]
    let config = AttrConfig::default();
    #[cfg(feature = "customise")]
    let config = AttrConfig::default().with_customisations(customisations);

    let (item_vis, item_name, attrs) = get_vis_name_attrs(&item)?;

    let docs = get_docs(attrs, config.trim)?
        .ok_or_else(|| Error::new_spanned(&item, "Missing doc comments"))?;

    let const_vis = config.custom_vis.unwrap_or(item_vis);
    let const_name = config
        .custom_name
        .unwrap_or_else(|| format!("{}_DOCS", item_name.to_case(Case::ScreamingSnake)));
    let const_ident = Ident::new(&const_name, Span::call_site());

    Ok(quote! {
        #item
        #const_vis const #const_ident: &'static str = #docs;
    })
}
