//! Tests for derive macros.
//!
//! The tests for `*Opt` macros are deliberately incomplete, because they share
//! the vast majority of their implementation with their respective non-opt
//! counterparts.
//!
//! `fail_*` modules contains examples that should fail to compile,
//! tested with doctest's `compile_fail` feature.
//! Some `compile_fail` examples may be preceded by a "base" working example,
//! which is useful for preventing some false negatives.
//! I.e. the `compile_fail` examples fail, but it's due to other errors
//! that we are not trying to test for.

#[cfg(test)]
mod documented;
#[cfg(test)]
mod documented_fields;
#[cfg(test)]
mod documented_fields_opt;
#[cfg(test)]
mod documented_opt;
#[cfg(test)]
mod documented_variants;
#[cfg(test)]
mod documented_variants_opt;

mod fail_documented;
mod fail_documented_fields;
mod fail_documented_variants;
