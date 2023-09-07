use proc_macro::{TokenStream, TokenTree};
use quote::{quote};

#[proc_macro]
pub fn btreemap_proc(input: TokenStream) -> TokenStream {
    let map_init = quote!(
        let mut __temp_map = BTreeMap::new();
    );

    let tree_inserts_template = quote!(
        __temp_map.insert
    );

    let tree_return = quote!(
        __temp_map
    );

    let mut tree_inserts = quote!();

    for x in input {
        if let TokenTree::Group(group) = x {
            let group = proc_macro2::TokenStream::from(group.stream());

            tree_inserts.extend(tree_inserts_template.clone());
            tree_inserts.extend(quote!((#group);));

        }
    }

    TokenStream::from(quote!(
            {
                #map_init
                #tree_inserts
                #tree_return
            }
        ))
}





