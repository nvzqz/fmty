#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![no_std]

#[cfg(test)]
extern crate alloc;

#[macro_use]
mod macros;

mod concat;
mod cond;
mod convert_case;
mod fmt_with;
mod format_args;
mod infix;
mod join;
mod once;
mod quote;
mod repeat;
mod truncate;

/// Types defined by this crate.
///
/// These are provided in a separate module in order to make the crate root's
/// documentation easier to navigate.
pub mod types {
    #[doc(inline)]
    pub use crate::{
        concat::types::*, cond::types::*, convert_case::types::*,
        fmt_with::types::*, infix::types::*, join::types::*, repeat::types::*,
        truncate::types::*,
    };
}

pub use crate::{
    concat::*, cond::*, convert_case::*, fmt_with::*, format_args as fmt_args,
    infix::*, join::*, quote::*, repeat::*, truncate::*,
};
