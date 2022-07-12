use proc_macro::TokenStream;
use vec_walk_dir::expand_vec_walk_dir;

mod vec_walk_dir;

#[proc_macro]
pub fn vec_walk_dir(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);
    expand_vec_walk_dir(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
