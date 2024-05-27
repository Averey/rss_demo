use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Table, attributes(col))]
pub fn table_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    //eprintln!("{:#?}", ast);
    let ident = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    //fields.iter().for_each(|f| {
    //    let attr = f.attrs
    //
    //});

    let fstr = fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            name.as_ref().unwrap().to_string()
        })
        .collect::<Vec<String>>();

    //eprintln!("{:#?}", fields);
    eprintln!("{:#?}", fstr);

    let expanded = quote! {

        //use db_derive::table::DbTable;
        //
        //
        impl db::table::DbTable for #ident
        {

        }
    };
    expanded.into()
}
