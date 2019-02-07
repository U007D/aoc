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
    hash::Hash,
    iter::FromIterator,
    num::ParseIntError,
    result::Result as StdResult,
    str::FromStr
};

use num_traits::PrimInt;

pub use {
    consts::*,
    error::Error,
};

mod consts;
#[cfg(test)]
mod unit_tests;

pub mod error;

pub type Result<T> = StdResult<T, Error>;

pub fn find_first_repeating_frequency<I, V>(deltas: I) -> Result<V>
                                                       where I: IntoIterator<Item = Result<V>>,
                                                             I::IntoIter: Clone,
                                                             V: Default + Eq + Hash + PrimInt, {
    let mut freqs = HashSet::<V>::from_iter([V::default()].iter().cloned());
    freq_iter(deltas).find(|res| !freqs.insert(*res.as_ref().unwrap_or(&V::default())))
                     .expect(msg::ERR_INTERNAL_EXHAUSTED_DELTA_VALUES)
}

fn freq_iter<I, V>(deltas: I) -> impl Iterator<Item = Result<V>>
                              where I: IntoIterator<Item = Result<V>>,
                                    I::IntoIter: Clone,
                                    V: Default + PrimInt, {
    let init = Ok(V::default());
    deltas.into_iter()
          .cycle()
          .scan(init, |freq, delta| {
              *freq = freq.as_ref()
                          .and_then(|ref f| delta.as_ref()
                                                 .and_then(|ref d| f.checked_add(&d)
                                                                    .ok_or_else(|| &Error::Overflow)));
              Some(freq.clone())
          })
}

pub fn strings_to_values<S, V>(string_deltas: S) -> impl Iterator<Item = Result<V>>
                                                 where S: IntoIterator<Item = String>,
                                                       V: Default + FromStr<Err = ParseIntError> + PrimInt, {
    string_deltas.into_iter()
                 .map(|s| s.parse::<V>().map_err(Error::InvalidInputError))
}
