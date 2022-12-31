#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![no_std]

mod concat;
mod fmt_with;

/// Private helper types and traits. Not for external use.
#[doc(hidden)]
pub mod _priv;

pub use crate::{concat::*, fmt_with::*};
