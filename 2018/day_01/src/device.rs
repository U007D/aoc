use std::{
    borrow::Borrow,
    collections::HashSet,
    iter::FromIterator
};

use crate::{
    Error,
    Result
};

pub struct Device;

impl Device {
    pub fn calc_final_frequency<I>(deltas: I) -> Result<i32>
                                                 where I: IntoIterator,
                                                       I::Item: Borrow<i32>, {
        deltas.into_iter()
              .try_fold(0_i32, |sum, delta| sum.checked_add(*delta.borrow()))
              .ok_or(Error::Overflow)
    }

    pub fn find_first_duplicate_frequency<I>(deltas: I) -> Result<i32>
                                                           where I: IntoIterator,
                                                                 I::IntoIter: Clone,
                                                                 I::Item: Borrow<i32>,  {
        let mut seen = HashSet::<i32>::from_iter([0_i32].iter().cloned());
        Device::deltas_as_frequency_stream(deltas).find(|freq| !seen.insert(*freq.as_ref().unwrap_or(&0)))
                                                  .unwrap_or(Err(Error::ExhaustedDeltaValues))
    }

    fn deltas_as_frequency_stream<I>(deltas: I) -> impl Iterator<Item = Result<i32>>
                                                   where I: IntoIterator,
                                                         I::IntoIter: Clone,
                                                         I::Item: Borrow<i32>, {
        deltas.into_iter()
              .cycle()
              .scan(Ok(0_i32), |freq, delta| {
                  Some(freq.clone()
                           .and_then(|f| {
                               *freq = f.checked_add(*delta.borrow())
                                        .ok_or(Error::Overflow);
                               freq.clone()
                           }))
              })
    }
}
