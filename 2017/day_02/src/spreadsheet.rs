use consts::*;
use super::{Error, Result};
use ::std::cmp::{max, min};
use ::std::num::ParseIntError;
use ::std::result::Result as StdResult;

pub type CellType = i32;

#[derive(Clone, Default, Debug)]
pub struct Spreadsheet {
    sheet: Vec<Vec<CellType>>,
}

impl Spreadsheet {
    pub fn new(data: &str) -> Result<Self> {
        Ok(Self {
            sheet: Self::make_sheet(data)?,
        })
    }

    pub fn checksum(&self) -> CellType {
        self.sheet
            .iter()
            .map(|row| {
                row.iter()
                   .fold((None, None), |acc, el| {
                       let (mut row_min, mut row_max) = acc;
                       row_min = match row_min {
                           Some(curr) => Some(min(curr, el)),
                           None => Some(el),
                       };
                       row_max = match row_max {
                           Some(curr) => Some(max(curr, el)),
                           None => Some(el),
                       };
                       (row_min, row_max)
                   })
            })
            .map(|extremes| match extremes {
               (Some(row_min), Some(row_max)) => row_max - row_min,
               _ => panic!(MSG_ERR_INTERNAL_NONE_ERROR),
            })
            .sum()
    }

    pub fn find_evenly_divisible(&self) -> Option<CellType> {
        let foo = self.sheet
                  .iter()
                  .map(|row| {
                      row.iter()
                          .enumerate()
                          .skip(1)
                          .filter_map(|(i, ref el)| {
                              match row[..i].iter()
                                            .skip_while(|prev_el| {
                                                let larger = max(**prev_el, **el);
                                                let smaller = min(**prev_el, **el);
                                                match smaller {
                                                    0 => true,
                                                    _ => match larger % smaller {
                                                        0 => false,
                                                        _ => true,
                                                    },
                                                }})
                                            .next() {
                                  Some(v) => Some(max(v, el) - min(v, el)),
                                  None => None,
                              }
                          })
                  })
                  .collect::<Vec<_>>();
        None
/*                      {
            v if v.len() == 0 => None,
            v => Some(v.iter().sum()),
        }
*/    }

    fn make_sheet(data: &str) -> Result<Vec<Vec<CellType>>> {
        if data.trim().is_empty() { Err(Error::NoImportData)? }

        let rows = data.split('\n')
                       .collect::<Vec<_>>();
        Ok(rows.iter()
            .map(|s| s.split_whitespace()
                      .map(|v| v.parse::<CellType>())   // Result<elements>
                      .collect::<StdResult<Vec<_>, ParseIntError>>()) // Result<rows>
            .collect::<StdResult<Vec<_>, ParseIntError>>()?) // Result<sheet>
    }
}
