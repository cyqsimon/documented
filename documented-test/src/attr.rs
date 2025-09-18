//! Tests for attribute macros.
//!
//! `fail_*` modules contains examples that should fail to compile,
//! tested with doctest's `compile_fail` feature.
//! Some `compile_fail` examples may be preceded by a "base" working example,
//! which is useful for preventing some false negatives.
//! I.e. the `compile_fail` examples fail, but it's due to other errors
//! that we are not trying to test for.

#[cfg(test)]
mod docs_const;

mod fail_docs_const;
