//! # Caveats
//!
//! ## No line-trimming for macro-inserted doc comments
//!
//! The line-trimming feature does not work for doc comments inserted by macros:
//!
//! ```
//! #[doc = concat!("  line 1\n", "    line 2")]
//! #[derive(documented::Documented)]
//! struct Terrible;
//!
//! # use documented::Documented;
//! // trim is enabled (by default here) but does not work
//! assert_eq!(Terrible::DOCS, "  line 1\n    line 2");
//! ```
//!
//! This is because the expansion of your macro invocation
//! (e.g. `concat!`, `include_str!`, etc.) is not visible from the perspective
//! of the procedural macros of `documented`. Therefore it is not possible
//! (or rather, not practical) to do any post-processing on the text contents.
//!
//! Note that If an item has multiple `#[doc = ...]` attributes and only
//! a subset of them use the macro instead of literal form, line-trimming
//! will still work for the literal form attributes:
//!
//! ```
//! ///     line 1
//! #[doc = concat!("  line 2\n", "    line 3")]
//! #[doc = "line 4    "]
//! #[derive(documented::Documented)]
//! struct Terrible;
//!
//! # use documented::Documented;
//! // trim does not work for line 2 & 3
//! assert_eq!(Terrible::DOCS, "line 1\n  line 2\n    line 3\nline 4");
//! ```
