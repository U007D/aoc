use std::{
    collections::HashSet,
    iter::FromIterator
};

use crate::{
    Error,
    Result
};

pub struct Device;

impl Device {
    pub fn calc_final_frequency<'a>(mut deltas: impl Iterator<Item = &'a i32>) -> Result<i32> {
        deltas
            .try_fold(0_i32, |sum, delta| sum.checked_add(*delta))
            .ok_or(Error::Overflow)
    }

    pub fn first_duplicate_frequency<'a, I>(deltas: I) -> Result<i32>
                                            where I: Iterator<Item = &'a i32> + Clone +'a, {
        let mut seen = HashSet::<i32>::from_iter([0_i32].iter().cloned());
        #[allow(clippy::maybe_infinite_iter)]
        deltas.cycle()
              .scan(Ok(0_i32), |freq, &delta| {
                  Some(freq.clone()
                           .and_then(|f| {
                               *freq = f.checked_add(delta)
                                        .ok_or(Error::Overflow);
                               freq.clone()
                           }))
              })
              .find(|freq| !seen.insert(*freq.as_ref().unwrap_or(&0)))
                                .unwrap_or(Err(Error::ExhaustedDeltaValues))
    }
}
