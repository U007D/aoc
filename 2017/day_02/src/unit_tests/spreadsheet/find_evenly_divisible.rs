use spreadsheet::{CellType, Spreadsheet};
use super::{given, run,
            Result};

#[derive(Clone, Debug)]
struct Env {
    spreadsheet: Result<Spreadsheet>,
    result: Option<Option<CellType>>,
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
    run(&given("a spreadsheet without evenly divisible elements", Env::new("2, 3, 5, 7, 11"), |ctx| {
        ctx.when("evenly divisible elements are sought", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.spreadsheet.clone().unwrap().find_evenly_divisible());
            });
            let expected_result = None;

            ctx.then("the result should be None", move |env| {
                assert!(env.result.unwrap() == expected_result);
            });
        });
    }));
}

