#![doc = include_str!("../README.md")]
#![doc(test(attr(deny(warnings))))]
#![no_std]

mod concat;
mod fmt_with;

pub use crate::{concat::*, fmt_with::*};
