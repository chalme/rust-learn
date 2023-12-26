use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  println!("{:?}", input);
  let input = parse_macro_input!(input as DeriveInput);

  impl_hello_macro(&input)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
      impl HelloMacro for #name {
          fn hello_macro() {
              println!("Hello, Macro! My name is {}!", stringify!(#name));
          }
      }
  };
  gen.into()
}

fn add(left: usize, right: usize) -> usize {
  left + right
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
