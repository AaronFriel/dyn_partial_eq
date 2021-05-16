extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, parse_quote};

#[proc_macro_derive(DynPartialEq)]
pub fn dyn_partial_eq_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();

  impl_dyn_partial_eq(&ast)
}

fn impl_dyn_partial_eq(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
      impl DynPartialEq for #name {
          fn as_any(&self) -> &dyn core::any::Any {
            self
          }
          fn box_eq(&self, other: &dyn core::any::Any) -> bool {
            other.downcast_ref::<Self>().map_or(false, |a| self == a)
          }
      }
  };
  gen.into()
}


#[proc_macro_attribute]
pub fn dyn_partial_eq(_: TokenStream, input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as syn::ItemTrait);

  let name = &input.ident;

  let bound: syn::TypeParamBound = parse_quote! {
    dyn_partial_eq::DynPartialEq
  };

  input.supertraits.push(bound);

  (quote! {
    #input

    impl core::cmp::PartialEq for Box<dyn #name> {
      fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
      }
    }

    impl core::cmp::PartialEq<&Self> for Box<dyn #name> {
      fn eq(&self, other: &&Self) -> bool {
        self.box_eq(other.as_any())
      }
    }
  }).into()
}
