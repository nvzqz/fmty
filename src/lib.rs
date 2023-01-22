#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(not(test), no_std)]

#[macro_use]
mod macros;

mod concat;
mod cond;
mod convert_case;
mod fmt_iterator;
mod fmt_with;
mod format_args;
mod infix;
mod join;
mod noop;
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
        fmt_with::types::*, infix::types::*, join::types::*, noop::types::*,
        repeat::types::*, truncate::types::*,
    };
}

pub use crate::{
    concat::*, cond::*, convert_case::*, fmt_iterator::*, fmt_with::*,
    format_args as fmt_args, infix::*, join::*, noop::*, quote::*, repeat::*,
    truncate::*,
};
