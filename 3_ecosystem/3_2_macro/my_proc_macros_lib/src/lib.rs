use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::{quote};

#[proc_macro]
pub fn btreemap_proc(input: TokenStream) -> TokenStream {
    let map_name = {
        let name = input.clone().into_iter().next().unwrap();
        if let TokenTree::Ident(ident) = name {
            ident
        } else {
            panic!("Expected identifier as first argument");
        }
    };
    let map_name = proc_macro2::Ident::new(&map_name.to_string(), Span::from(map_name.span()));

    let tree_inserts_template = quote!(
        .insert
    );

    let mut tree_inserts = quote!();

    for x in input {
        if let TokenTree::Group(group) = x {
            let group = proc_macro2::TokenStream::from(group.stream());

            tree_inserts.extend(quote!(#map_name));
            tree_inserts.extend(tree_inserts_template.clone());
            tree_inserts.extend(quote!((#group);));

        }
    }

    TokenStream::from(quote!(
            #tree_inserts
        ))
}





