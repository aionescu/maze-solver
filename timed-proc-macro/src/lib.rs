use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn timed(_args: TokenStream, item: TokenStream) -> TokenStream {
  if cfg!(not(feature = "enabled")) {
    return item
  }

  let ItemFn{attrs, vis, sig, block} = syn::parse_macro_input!(item as ItemFn);
  let name = &sig.ident.to_string();

  quote! {
    #(#attrs)*
    #vis #sig {
      let mut body = move || #block;

      let time = std::time::Instant::now();
      let r = body();
      let elapsed = time.elapsed();

      println!("{}: {:?}", #name, elapsed);
      r
    }
  }.into()
}
