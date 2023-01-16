#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![no_std]

#[cfg(test)]
extern crate alloc;

mod concat;
mod convert_case;
mod fmt_with;
mod format_args;
mod infix;
mod join;
mod quote;
mod repeat;
mod truncate;

/// Private helper types and traits. Not for external use.
#[doc(hidden)]
pub mod _priv;

pub use crate::{
    concat::*, convert_case::*, fmt_with::*, format_args as fmt_args, infix::*,
    join::*, quote::*, repeat::*, truncate::*,
};
