use std::collections::HashMap;

pub struct Device;

impl Device {
    pub fn checksum(ids: impl IntoIterator<Item = String>) -> Option<usize> {
        let parts = ids.into_iter()
                       .map(|id| Device::id_histogram(id)
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

    fn id_histogram(id: String) -> HashMap<char, usize> {
        id.chars()
          .fold(HashMap::new(), |mut histo, id| {
              histo.entry(id)
                   .and_modify(|count| *count += 1)
                   .or_insert(1);
              histo
          })
    }
}
