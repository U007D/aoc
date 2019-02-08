#![warn(clippy::all)]
#![forbid(unsafe_code)] // Do not remove!  Explicitly change to #![allow(unsafe_code)] to use `unsafe` keyword.
#![forbid(overflowing_literals)]
//#![deny(warnings)]
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
//#![warn(clippy::multiple_crate_versions, clippy::print_stdout, clippy::unimplemented, clippy::use_debug, missing_docs)]
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used)]
// ^^^ End of safety-critical lint section ^^^
#![allow(clippy::match_bool,)]

use day_02a::{
    Device,
    Result,
};
use std::{
    env,
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
};
fn read_ids(filename: String) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
              .lines()
              .map(|line| Ok(line?))
              .collect()
}

fn main() -> Result<()> {
    let args_fname = env::var("CARGO_MANIFEST_DIR")? + "/puzzle_input.nsv";
    let ids = read_ids(args_fname)?;
    println!("First repeated frequency value is {}.", Device::checksum(ids));
    Ok(())
}
