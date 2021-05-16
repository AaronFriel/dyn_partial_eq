#![no_std]
use core::any::Any;

pub use dyn_partial_eq_derive::*;

pub trait DynPartialEq {
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}
