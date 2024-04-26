use darling::{FromDeriveInput, FromMeta};


#[derive(FromDeriveInput)]
#[darling(attributes(filterable_enum), forward_attrs(allow, doc, cfg))]
pub struct Opts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    pub repr: Option<String>,
    #[darling(multiple)]
    pub kind_extra_derive: Vec<syn::Ident>,
}
