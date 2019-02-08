//! --- Part Two ---
//!
//! You notice that the device repeats the same frequency change list over and over. To calibrate the device, you need
//! to find the first frequency it reaches twice.
//!
//! For example, using the same list of changes above, the device would loop as follows:
//!
//!   Current frequency  0, change of +1; resulting frequency  1.
//!   Current frequency  1, change of -2; resulting frequency -1.
//!   Current frequency -1, change of +3; resulting frequency  2.
//!   Current frequency  2, change of +1; resulting frequency  3.
//!   (At this point, the device continues from the start of the list.)
//!   Current frequency  3, change of +1; resulting frequency  4.
//!   Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
//!
//! In this example, the first frequency reached twice is 2. Note that your device might need to repeat its list of
//! frequency changes many times before a duplicate frequency is found, and that duplicates might be found while in the
//! middle of processing the list.
//!
//! Here are other examples:
//!
//!   +1, -1 first reaches 0 twice.
//!   +3, +3, +4, -2, -4 first reaches 10 twice.
//!   -6, +3, +8, +5, -6 first reaches 5 twice.
//!   +7, +7, -2, -7, -4 first reaches 14 twice.
//!
//! What is the first frequency your device reaches twice?

#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
//#![deny(warnings)]
//#![deny(missing_docs)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]

use std::{
    collections::HashSet,
    iter::FromIterator,
    result::Result as StdResult,
};

pub use {
    consts::*,
    error::Error,
};

mod consts;
#[cfg(test)]
mod unit_tests;
mod error;
pub type Result<T> = StdResult<T, Error>;

pub struct Device;

impl Device {
    pub fn first_duplicate_frequency<I>(deltas: I) -> Result<i32>
                                                   where I: IntoIterator<Item = i32>,
                                                         I::IntoIter: Clone, {
        let mut seen = HashSet::<i32>::from_iter([0_i32].iter().cloned());
        Device::frequency_stream(deltas).find(|freq| !seen.insert(*freq.as_ref().unwrap_or(&0)))
                                        .unwrap_or(Err(Error::ExhaustedDeltaValues))
    }

    fn frequency_stream<I>(deltas: I) -> impl Iterator<Item = Result<i32>>
                                      where I: IntoIterator<Item = i32>,
                                            I::IntoIter: Clone, {
        deltas.into_iter()
              .cycle()
              .scan(Ok(0_i32), |freq, delta| {
                  Some(freq.clone()
                           .and_then(|f| {
                               *freq = f.checked_add(delta)
                                        .ok_or(Error::Overflow);
                               freq.clone()
                           }))
              })

    }
}
