use proc_macro::TokenStream;
use quote::quote;
use syn::{self};

#[proc_macro_derive(TableMacro)]
pub fn table_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_table_macro(&ast)
}

fn impl_table_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TableMacro for #name {
            fn table_name() {
                println!("table name: {}", stringify!(#name));
            }

        }
    };
    gen.into()
}