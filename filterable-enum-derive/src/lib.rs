// MIT License

// Copyright (c) 2022 Gino Valente
// Copyright (c) 2024 Levi Zim

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::Data;

#[proc_macro_derive(FilterableEnum)]
pub fn derive_filterable_enum(ts: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(ts as syn::DeriveInput);

    let Data::Enum(data) = input.data else {
        return syn::Error::new_spanned(
            input,
            "Cannot derive FilterableEnum on this type, expected enum",
        )
        .to_compile_error()
        .into();
    };

    let vis = &input.vis;
    let ident = &input.ident;
    let kinds = data
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect::<Vec<_>>();
    let filterable_enum = get_crate("filterable-enum");
    let ident_kind = format_ident!("{}Kind", ident);
    let ident_filterable = format_ident!("Filterable{}", ident);

    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    TokenStream::from(quote::quote! {
        // Create EnumKind
        #[#filterable_enum::enumflags2::bitflags]
        #[repr(u32)]
        #[derive(Debug, PartialEq, Clone, Copy)]
        #vis enum #ident_kind {
            #(#kinds,)*
        }

        impl #filterable_enum::EnumFilter<#ident_kind> for #filterable_enum::enumflags2::BitFlags<#ident_kind> {
            fn contains(&self, id: #ident_kind) -> bool {
                self.intersects(id)
            }
        }

        #vis struct #ident_filterable #ty_generics {
            inner: #ident #ty_generics,
            id: #ident_kind,
        }

        impl #impl_generics #filterable_enum::FilterableEnum<#ident #ty_generics> for #ident_filterable #where_clause {
            type Id = #ident_kind;
            type Filter = #filterable_enum::enumflags2::BitFlags<#ident_kind>;

            fn filterable_id(&self) -> Self::Id {
                self.id
            }

            fn filter_ref(&self, filter: impl Into<Self::Filter>) -> Option<&#ident> {
                if filter.into().contains(self.id) {
                    Some(&self.inner)
                } else {
                    None
                }
            }

            fn filter_and_take(self, filter: impl Into<Self::Filter>) -> Option<#ident> {
                if filter.into().contains(self.id) {
                    Some(self.inner)
                } else {
                    None
                }
            }
        }

        impl #impl_generics From<#ident #ty_generics> for #ident_filterable #ty_generics {
            fn from(inner: #ident #ty_generics) -> Self {
                let id = match inner {
                    #(
                        #ident::#kinds(_) => #ident_kind::#kinds,
                    )*
                };
                #ident_filterable { inner, id }
            }
        }
    })
}

fn get_crate(name: &str) -> proc_macro2::TokenStream {
    let found_crate =
        crate_name(name).unwrap_or_else(|_| panic!("`{}` not found in `Cargo.toml`", name));

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = format_ident!("{}", &name);
            quote!( #ident )
        }
    }
}
