#![cfg_attr(feature = "clippy", feature(plugin))] // nightly rustc required by `clippy`
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(match_bool)] // disable false positives
#![warn(clone_on_ref_ptr, decimal_literal_representation, doc_markdown, else_if_without_else, empty_enum,
        enum_glob_use, expl_impl_clone_on_copy, fallible_impl_from, filter_map, if_not_else, inline_always,
        invalid_upcast_comparisons, int_plus_one, invalid_upcast_comparisons, items_after_statements, linkedlist,
        match_same_arms, mem_forget,
        /* multiple_crate_versions /* enable before ship to discover and reconcile multiple crate versions */, */
        mut_mut, missing_debug_implementations, mut_mut, mutex_integer, needless_borrow, needless_continue,
        nonminimal_bool, option_map_unwrap_or, option_map_unwrap_or_else, option_map_unwrap_or_else,
        option_unwrap_used, pub_enum_variant_names, range_plus_one, replace_consts, redundant_closure,
        result_map_unwrap_or_else, result_unwrap_used, shadow_unrelated, similar_names, stutter, trivial_casts,
        non_camel_case_types, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix, use_self,
        used_underscore_binding, unused_import_braces, unnecessary_mut_passed, unused_qualifications,
        wrong_pub_self_convention)]
// Safety-critical application lints (pedantic--use for safety-critical applications only)
#![deny(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, float_cmp_const,
        /*indexing_slicing,*/  integer_arithmetic, maybe_infinite_iter)]
#![forbid(overflowing_literals, unused_must_use)]
// End of safety-critical lint section

#![feature(associated_type_defaults, stmt_expr_attributes, try_trait, use_extern_macros)]

#[macro_use]
extern crate failure;
extern crate num_traits;
#[cfg(test)]
extern crate rspec;

#[cfg(test)]
#[macro_use]
extern crate hesl;

mod consts;
mod di;
mod error;
mod spreadsheet;
#[cfg(test)]
mod unit_tests;

use consts::*;
pub use di::Container;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
