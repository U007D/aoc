use spreadsheet::{CellType, Spreadsheet};
use super::{given, run,
            Error, Result};

#[derive(Clone, Debug)]
struct Env {
    spreadsheet: Result<Spreadsheet>,
    result: Option<CellType>,
}

impl Env {
    fn new(data: &str) -> Self {
        Self {
            spreadsheet: Spreadsheet::new(data),
            result: None,
        }
    }
}

#[test]
fn tests() {
    run(&given("a spreadsheet with no data", Env::new(""), |ctx| {
        let expected_result = Err::<Spreadsheet, Error>(Error::NoImportData);

        ctx.then("the result should be failure to instantiate due to no import data", move |env| {
            assert!(env.spreadsheet.clone().unwrap_err() == expected_result.clone().unwrap_err());
        });
    }));

    run(&given("a spreadsheet with invalid data", Env::new("hello world"), |ctx| {
        // Create Error::InvalidInt(ParseIntError(IntErrorKind::InvalidDigit)) error
        let expected_result = Err::<Spreadsheet, Error>(Error::InvalidInt("x".parse::<CellType>().unwrap_err()));

        ctx.then("the result should be failure to instantiate due to invalid integer", move |env| {
            assert!(env.spreadsheet.clone().unwrap_err() == expected_result.clone().unwrap_err());
        });
    }));
}
