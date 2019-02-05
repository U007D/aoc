#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
#![deny(warnings)]
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
    result::Result as StdResult
};

pub use {
    consts::*,
    error::Error,
};

mod consts;
pub mod error;

pub type Result<T> = StdResult<T, Error>;

pub fn find_first_repeating_freq(args: Vec<String>) -> Result<i32> {
    let mut freqs = HashSet::<i32>::new();
    match args.iter()
              .cycle()
              .map(|s| s.parse::<i32>()
                        .or_else(|e| Err(Error::InvalidInputError(e))))
              .try_fold(0_i32, |freq, delta| match delta {
                  Ok(v) => {
                      freq.checked_add(v)
                      .ok_or(Error::Overflow)
                      .and_then(|freq| {
                          match freqs.insert(freq) {
                              true => Ok(freq),
                              false => Err(Error::RepeatedFrequency(freq)),
                          }
                      })
                  },
                  e => e,
              }) {
        Err(Error::RepeatedFrequency(f)) => Ok(f),
        Err(e) => Err(e),
        Ok(_) => unreachable!(),
    }
}
