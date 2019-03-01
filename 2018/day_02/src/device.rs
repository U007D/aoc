use std::collections::HashMap;
use std::borrow::Borrow;

pub struct Device;

impl Device {
    pub fn checksum<I>(ids: I) -> Option<usize>
                                where I: IntoIterator,
                                      I::Item: Borrow<String>, {
        let parts = ids.into_iter()
                       .map(|id| Device::id_histogram(id.borrow())
                                        .into_iter()
                                        .fold((0_usize, 0_usize), |mut checksum, (_, count)| {
                                            match count {
                                                2 => checksum.0 = 1,
                                                3 => checksum.1 = 1,
                                                _ => (),
                                            };
                                            checksum
                                        })
                       )
                       .try_fold((0_usize, 0_usize),
                                 |checksum, (twos, threes)| match (checksum.0.checked_add(twos),
                                                                   checksum.1.checked_add(threes)) {
                                     (Some(twos), Some(threes)) => Some((twos, threes)),
                                     _ => None,
                                 });
        parts.and_then(|(twos, threes)| twos.checked_mul(threes))
    }

    fn id_histogram<S: AsRef<str>>(id: S) -> HashMap<char, usize> {
        id.as_ref()
          .chars()
          .fold(HashMap::new(), |mut histo, id| {
              histo.entry(id)
                   .and_modify(|count| *count += 1)
                   .or_insert(1);
              histo
          })
    }
}
