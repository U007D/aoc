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
pub use {
    consts::*,
    error::Error,
};
use std::result::Result as StdResult;

mod consts;
pub mod error;

pub type Result<T> = StdResult<T, Error>;

pub fn calc_freq_delta_from_args(args: Vec<String>) -> Result<i32> {
    args.iter()
        .map(|s| s.parse::<i32>()
                  .or_else(|e| Err(Error::InvalidInputError(e))))
        .try_fold(0_i32, |sum, el| match el {
            Ok(v) => sum.checked_add(v)
                        .ok_or(Error::Overflow),
            e => e,
        })
}
