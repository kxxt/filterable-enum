use darling::FromDeriveInput;

#[derive(FromDeriveInput)]
#[darling(attributes(filterable_enum), forward_attrs(allow, doc, cfg))]
pub struct Opts {
    #[allow(unused)]
    ident: syn::Ident,
    #[allow(unused)]
    attrs: Vec<syn::Attribute>,
    pub repr: Option<String>,
    #[darling(multiple)]
    pub kind_extra_derive: Vec<syn::Ident>,
    #[darling(multiple)]
    // TODO: there should be a better way to pass through the attributes
    pub kind_extra_attrs: Vec<String>,
}
