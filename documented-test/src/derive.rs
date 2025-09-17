//! Tests for derive macros.
//!
//! The tests for `*Opt` macros are deliberately incomplete, because they share
//! the vast majority of their implementation with their respective non-opt
//! counterparts.

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
