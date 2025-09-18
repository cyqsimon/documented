mod missing_docs {
    //! ```
    //! #[derive(documented::DocumentedVariants)]
    //! enum Terrible {
    //!     /// I'm deaf
    //!     AhMyEyes,
    //!     /// I'm blindfolded
    //!     AhMyEars,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! enum Terrible {
    //!     AhMyEyes,
    //!     /// I'm blindfolded
    //!     AhMyEars,
    //! }
    //! ```
}

#[cfg(feature = "customise")]
mod non_applicable_options {
    //! ```
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants()]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_variants()]
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants(vis = pub)]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_variants(vis = pub)]
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants(rename = "HalfOfYouKnow")]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_variants(rename = "HalfOfYouKnow")]
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants(rename_all = "snake_case")]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedVariants)]
    //! enum AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_variants(rename_all = "snake_case")]
    //!     Children,
    //!     /// They know.
    //!     Adults,
    //! }
    //! ```
}

#[cfg(feature = "customise")]
mod non_unique_options {
    //! ```
    //! /// If you can call them people.
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants()]
    //! enum NinetyNinePercentOf {
    //!     /// If you can call the people.
    //!     People,
    //!     /// They're barking outside.
    //!     Dogs,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants(
    //!     default = "If you can call them people.",
    //!     default = "Can you?",
    //! )]
    //! enum NinetyNinePercentOf {
    //!     /// If you can call the people.
    //!     People,
    //!     /// They're barking outside.
    //!     Dogs,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::DocumentedVariants)]
    //! enum NinetyNinePercentOf {
    //!     /// If you can call the people.
    //!     #[documented_variants(
    //!         default = "If you can call them people.",
    //!         default = "Can you?",
    //!     )]
    //!     People,
    //!     /// They're barking outside.
    //!     Dogs,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::DocumentedVariants)]
    //! #[documented_variants(trim = true, trim = false)]
    //! enum NinetyNinePercentOf {
    //!     /// If you can call the people.
    //!     People,
    //!     /// They're barking outside.
    //!     Dogs,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::DocumentedVariants)]
    //! enum NinetyNinePercentOf {
    //!     /// If you can call the people.
    //!     #[documented_variants(trim = true, trim = false)]
    //!     People,
    //!     /// They're barking outside.
    //!     Dogs,
    //! }
    //! ```
}
