mod missing_docs {
    //! ```compile_fail
    //! #[documented::docs_const]
    //! mod terrible {}
    //! ```
}

#[cfg(feature = "customise")]
mod non_applicable_options {
    //! ```
    //! /// None of them know it.
    //! #[documented::docs_const()]
    //! struct AsYouAllKnow;
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[documented::docs_const(rename_all = "snake_case")]
    //! struct AsYouAllKnow;
    //! ```
}

#[cfg(feature = "customise")]
mod non_unique_options {
    //! ```
    //! /// If you can call them people.
    //! #[documented::docs_const()]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[documented::docs_const(vis = pub, vis = pub(crate))]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[documented::docs_const(rename = "YouCantSayThat", rename = "YesHeCan")]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[documented::docs_const(default = "If you can call them people.", default = "Can you?")]
    //! struct NinetyNinePercentOfPeople;
    //! ```
    //!
    //! ```compile_fail
    //! /// If you can call them people.
    //! #[documented::docs_const(trim = true, trim = false)]
    //! struct NinetyNinePercentOfPeople;
    //! ```
}

mod illegal_item {
    //! ```compile_fail
    //! struct Horrible;
    //!
    //! /// Whatcha doin?
    //! #[documented::docs_const]
    //! impl Clone for Horrible {
    //!     fn clone(&self) -> Self {
    //!         Self
    //!     }
    //! }
    //! ```
}

mod inherited_private_visibility {
    //! ```compile_fail
    //! mod gotcha {
    //!     /// bitch
    //!     #[documented::docs_const]
    //!     mod vs {}
    //! }
    //! assert_eq!(gotcha::VS_DOCS, "bitch");
    //! ```
}

#[cfg(feature = "customise")]
mod private_visibility_override {
    //! ```
    //! mod gotcha {
    //!     /// bitch
    //!     #[documented::docs_const]
    //!     pub mod vs {}
    //! }
    //! assert_eq!(gotcha::VS_DOCS, "bitch");
    //! ```
    //!
    //! ```compile_fail
    //! mod gotcha {
    //!     /// bitch
    //!     #[documented::docs_const(vis = pub(self))]
    //!     pub mod vs {}
    //! }
    //! assert_eq!(gotcha::VS_DOCS, "bitch");
    //! ```
}
