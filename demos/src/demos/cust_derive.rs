extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

pub trait Caption {
    fn title(&self) -> String;
}

#[proc_macro_derive(Caption)]
pub fn caption_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_caption_macro(&ast)
}

fn impl_caption_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Dockable for #name {}
    };
    gen.into()
}
