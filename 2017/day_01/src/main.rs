#![warn(clippy::all)]
#![forbid(overflowing_literals,)]
#![deny(unsafe_code)] // Do not remove! Change to `allow` to explicitly opt-in to using `unsafe` (facilitates auditing)
// vvv Safety-critical application lints (pedantic: use for safety-critical applications only) vvv
#![deny(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss,
        clippy::cast_sign_loss, clippy::float_cmp_const, clippy::indexing_slicing, clippy::integer_arithmetic,
        clippy::maybe_infinite_iter, clippy::option_unwrap_used, clippy::result_unwrap_used,)]
// ^^^ End of safety-critical lint section ^^^
// Uncomment before ship to reconcile use of possibly redundant crates and uncover possible debug remnants
// #![warn(clippy::multiple_crate_versions, clippy::print_on_stdout, clippy::unimplemented, clippy::use_debug)]
#![allow(clippy::match_bool,)]
#![feature(never_type, try_trait, associated_type_defaults, self_in_typedefs)]

mod consts;
mod error;
#[cfg(test)]
mod unit_tests;
use self::error::Error;
use std::result::Result as StdResult;
type Result<T> = StdResult<T, Error>;

pub const BASE_10: u32 = 10;

fn sum_matching_digits_iter(init_val: (Option<u32>, Option<u32>), iter: impl Iterator<Item = char>) -> Option<u32> {
    iter.fold(init_val, |(sum, prev), curr| {
        match (curr.to_digit(BASE_10), prev) {
            (Some(curr_d), Some(prev_d)) => {
                let prev = Some(curr_d);
                match curr_d == prev_d {
                    true => (sum.and_then(|s: u32| {
                        match s.overflowing_add(curr_d) {
                            (s, false) => Some(s),
                            _ => None,
                        }
                    }), prev),
                    false => (sum, prev),
                }
            },
            curr_prev => (sum, curr_prev.0),
        }
    }).0
}

pub fn sum_matching_digits(digits: impl AsRef<str>) -> Option<u32> {
    sum_matching_digits_iter((Some(0), None), digits.as_ref()
                                      .chars()
                                      .chain({
                                          // Only add first element to end of digits if > 1 char in set
                                          let mut first_two = digits.as_ref().chars().take(2);
                                          first_two.nth(0)
                                                   .and_then(|c| first_two.nth(0)
                                                                          .and_then(|_| Some(c)))
                                      }))
}

fn main() -> Result<()> {
    let digits = "59945212267958384861888721899525514753529291453572849834636789447772281393981176491298438538371242283\
        536895511781293535483317797837429153613432291415383346882548197148136644392687919782155536777728388533288353454\
        847112297674777299484733912287764864566862651148756865369264986344956956922521593739716315435946569544941171492\
        946488766611575348519389339547876121464365711831444946799524523259892124812191396861381393149158527746287184435\
        324155247766428771317633594138229866193128628896894723977769686621487531877677937626541334293495153243338777879\
        254655415885849888271366763761288878191616724671425792619954827318789792845732465336888352263526911221698478329\
        435137589241942323459887267417892473791843197823877576131387428178263163762334435218578816782286948636819714454\
        426632514231841776289778999639199975294683549535486129666995267186491327899225845245566977151331633764632562251\
        818332576928213316655326812882169494512768444191542454234341418349139518545512533397855333959498151156228115659\
        992525552349445544739123596743798621824256951875934523637245915419927666513111752172181449986911218568829738251\
        623685641567269899399934129635368315931969976769929426735713361645359273712298232369372937823963182378797156129\
        563177151877573978153466354544121831986426375775286323938139645146813441628141225887958651697881216553533192337\
        988117967658524434247835524195414811321323444878357578884681965437368333429457188678554934224355113483437113116\
        243997444828323859985928647952719725775485849674339173222967529921277199644533764146655761968299456649418564937\
        687949119845374452272856577163179746494175865283954887899466899149727322882766653561798897835574818194546993543\
        175554176914948448128522325511897513864846384282968714361394896161929542677944412569297838396525192858352387361\
        429972451893638493564546456631513141248856619194514476289649967972477811968917871716481694278942827687762756891\
        241918117511355676923135716636372142986253676559695756998511213818728728757749991728395216178458473589662642911\
        753873744644255665144264991668133927686772333566467522733985418141425236514155213632674145648863798636993238872\
        78761615927993953372779567675";
    println!("The matching-digits sum of \"{}\" is {}", digits, sum_matching_digits(digits)?);
    Ok(())
}
