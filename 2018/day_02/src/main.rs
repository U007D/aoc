//! --- Day 2: Inventory Management System ---
//!
//! You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current
//! Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
//!
//! Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people
//! have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new
//! kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the
//! prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of
//! the project!"
//!
//! "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box
//! IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk
//! too far away to hear any more.
//!
//! Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were
//! discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates
//! (your puzzle input).
//!
//! To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID
//! containing exactly two of any letter and then separately counting those with exactly three of any letter. You can
//! multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
//!
//! For example, if you see the following box IDs:
//!
//!  abcdef contains no letters that appear exactly two or three times.
//!  bababc contains two a and three b, so it counts for both.
//!  abbcde contains two b, but no letter appears exactly three times.
//!  abcccd contains three c, but no letter appears exactly two times.
//!  aabcdd contains two a and two d, but it only counts once.
//!  abcdee contains two e.
//!  ababab contains three a and three b, but it only counts once.
//!
//! Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter
//! which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
//!
//! What is the checksum for your list of box IDs?
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

use std::{
    env,
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    result::Result as StdResult
};

pub use {
    consts::*,
    device::Device,
    error::Error,
};

mod consts;
mod device;
mod error;
#[cfg(test)]
mod unit_tests;

pub type Result<T> = StdResult<T, Error>;

fn read_ids(filename: String) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
              .lines()
              .map(|line| Ok(line?))
              .collect()
}

fn main() -> Result<()> {
    let args_fname = env::var("CARGO_MANIFEST_DIR")? + "/puzzle_input.nsv";
    let ids = read_ids(args_fname)?;
    println!("Day 2 part 1: First repeated frequency value is {}.", Device::checksum(&ids).ok_or(Error::Overflow)?);
    Ok(())
}
