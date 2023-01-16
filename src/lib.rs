#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![no_std]

#[cfg(test)]
extern crate alloc;

#[macro_use]
mod macros;

mod concat;
mod convert_case;
mod fmt_with;
mod format_args;
mod infix;
mod join;
mod once;
mod quote;
mod repeat;
mod truncate;

pub use crate::{
    concat::*, convert_case::*, fmt_with::*, format_args as fmt_args, infix::*,
    join::*, quote::*, repeat::*, truncate::*,
};
