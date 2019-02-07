use std::{
    collections::HashSet,
    hash::Hash,
    iter::FromIterator,
    num::ParseIntError,
    str::FromStr
};

use num_traits::PrimInt;

use crate::{
    Error,
    Result
};

#[cfg(test)]
mod unit_tests;

pub struct TimeDevice;

impl TimeDevice {
    pub fn first_duplicate_frequency<I, V>(deltas: I) -> Result<V> where I: IntoIterator<Item = Result<V>>,
                                                                         I::IntoIter: Clone,
                                                                         V: Default + Eq + Hash + PrimInt, {
        let mut past_frequencies = HashSet::<V>::from_iter([V::default()].iter().cloned());
        TimeDevice::frequency_stream(deltas)
                   .find(|res| !past_frequencies.insert(*res.as_ref().unwrap_or(&V::default())))
                   .unwrap_or(Err(Error::ExhaustedDeltaValues))
    }

    fn frequency_stream<I, V>(deltas: I) -> impl Iterator<Item = Result<V>> where I: IntoIterator<Item = Result<V>>,
                                                                              I::IntoIter: Clone,
                                                                              V: Default + PrimInt, {
        deltas.into_iter()
            .cycle()
            .scan(Ok(V::default()), |freq, delta| {
                                        *freq = freq.clone()
                                                    .and_then(|f| delta.and_then(|d| f.checked_add(&d)
                                                                       .ok_or_else(|| Error::Overflow)));
                                        Some(freq.clone())
                                    })
    }

    pub fn strings_to_result_deltas<S, V>(strings: S) -> impl Iterator<Item = Result<V>> + Clone
                                           where S: IntoIterator<Item = String>,
                                                 S::IntoIter: Clone,
                                                 V: FromStr<Err = ParseIntError> + PrimInt, {
        strings.into_iter()
               .map(|s| s.parse::<V>()
                         .map_err(Error::InvalidInputError))
    }
}
