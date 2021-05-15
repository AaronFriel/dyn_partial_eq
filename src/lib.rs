//! # dyn_partial_eq
//!
//! **PartialEq for trait objects**
//!
//! This crate provides macros for deriving PartialEq for `Box<dyn Trait>`, removing the boilerplate of having to provide your own implementations. Inspired by this blog post: https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5
//!
//! Add the crate to your project:
//!
//! ```sh
//! cargo add dyn_partial_eq
//! ```
//!
//! ```
//! use dyn_partial_eq::*;
//!
//! // Use this to add a DynPartialEq supertrait and implement PartialEq for your trait.
//! #[dyn_partial_eq]
//! trait SomeTrait {}
//!
//! //Derive DynPartialEq and PartialEq on your types that implement your trait.
//! #[derive(DynPartialEq, PartialEq)]
//! struct A(usize);
//! impl SomeTrait for A {}
//!
//! #[derive(DynPartialEq, PartialEq)]
//! struct B((usize, usize));
//! impl SomeTrait for B {}
//! ```
//!
//! And voila:
//!
//! ```
//! let boxed_a_zero: Box<dyn SomeTrait> = Box::new(A(0));
//! let boxed_a_one: Box<dyn SomeTrait> = Box::new(A(1));
//! let boxed_b: Box<dyn SomeTrait> = Box::new(B((1, 2)));
//!
//! assert_eq!(&boxed_a_zero == &boxed_a_zero, true);
//! assert_eq!(&boxed_a_zero == &boxed_a_one, false);
//! assert_eq!(&boxed_a_zero == &boxed_b, false);
//!
//! assert_eq!(&boxed_a_one == &boxed_a_zero, false);
//! assert_eq!(&boxed_a_one == &boxed_a_one, true);
//! assert_eq!(&boxed_a_one == &boxed_b, false);
//!
//! assert_eq!(&boxed_b == &boxed_a_zero, false);
//! assert_eq!(&boxed_b == &boxed_a_one, false);
//! assert_eq!(&boxed_b == &boxed_b, true);
//! ```

use std::any::Any;

pub use dyn_partial_eq_derive::*;

pub trait DynPartialEq {
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}
