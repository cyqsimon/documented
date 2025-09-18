mod missing_docs {
    //! ```
    //! #[derive(documented::Documented)]
    //! /// Frankly
    //! struct Terrible;
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::Documented)]
    //! struct Terrible;
    //! ```
}

#[cfg(feature = "customise")]
mod non_applicable_options {
    //! ```
    //! /// None of them know it.
    //! #[derive(documented::Documented)]
    //! #[documented()]
    //! struct AsYouAllKnow;
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[derive(documented::Documented)]
    //! #[documented(vis = pub)]
    //! struct AsYouAllKnow;
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[derive(documented::Documented)]
    //! #[documented(rename = "HalfOfYouKnow")]
    //! struct AsYouAllKnow;
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[derive(documented::Documented)]
    //! #[documented(rename_all = "snake_case")]
    //! struct AsYouAllKnow;
    //! ```
}

#[cfg(feature = "customise")]
mod non_unique_options {
    //! ```
    //! /// If you can call them people.
    //! #[derive(documented::Documented)]
    //! #[documented()]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::Documented)]
    //! #[documented(default = "If you can call them people.", default = "Can you?")]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[derive(documented::Documented)]
    //! #[documented(trim = true, trim = false)]
    //! struct NinetyNinePercentOfPeople;
    //! ```
}
