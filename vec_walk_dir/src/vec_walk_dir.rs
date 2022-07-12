use proc_macro2::TokenStream;
use quote::quote;
use walkdir::{DirEntry, WalkDir};

pub fn expand_vec_walk_dir(input: syn::LitStr) -> syn::Result<TokenStream> {
    let root = input.value();

    let mut intermediate = Vec::new();
    for path in WalkDir::new(root.clone())
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            if e.metadata().map(|m| m.is_file()).unwrap_or(false) {
                Some(e.into_path())
            } else {
                None
            }
        })
    {
        if let Ok(path) = path.strip_prefix(root.clone()) {
            if let Some(str) = path.to_str() {
                let lit = proc_macro2::Literal::string(str);
                intermediate.push(quote! { v.push(#lit); });
            }
        }
    }
    let intermediate = intermediate.into_iter().collect::<TokenStream>();

    Ok(quote! {
        {
            let mut v = Vec::new();
            #intermediate
            v
        }
    })
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
