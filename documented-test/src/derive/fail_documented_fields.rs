mod missing_docs {
    //! ```
    //! #[derive(documented::DocumentedFields)]
    //! struct Terrible {
    //!     /// I'm deaf
    //!     ah_my_eyes: u8,
    //!     /// I'm blindfolded
    //!     ah_my_ears: u8,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! struct Terrible {
    //!     ah_my_eyes: u8,
    //!     /// I'm blindfolded
    //!     ah_my_ears: u8,
    //! }
    //! ```
}

#[cfg(feature = "customise")]
mod non_applicable_options {
    //! ```
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields()]
    //! struct AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_fields()]
    //!     children: u8,
    //!     /// They know.
    //!     adults: u64,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields(vis = pub)]
    //! struct AsYouAllKnow {
    //!     /// None of them know it.
    //!     children: u8,
    //!     /// They know.
    //!     adults: u64,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! /// None of them know it.
    //! #[derive(documented::DocumentedFields)]
    //! struct AsYouAllKnow {
    //!     /// None of them know it.
    //!     #[documented_fields(vis = pub)]
    //!     children: u8,
    //!     /// They know.
    //!     adults: u64,
    //! }
    //! ```
}

#[cfg(feature = "customise")]
mod non_unique_options {
    //! ```
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields()]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     #[documented_fields()]
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields(rename_all = "snake_case", rename_all = "camelCase")]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     #[documented_fields(rename_all = "snake_case", rename_all = "camelCase")]
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     #[documented_fields(rename = "YouCantSayThat", rename = "YesHeCan")]
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields(
    //!     default = "If you can call them people.",
    //!     default = "Can you?",
    //! )]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     #[documented_fields(
    //!         default = "If you can call them people.",
    //!         default = "Can you?",
    //!     )]
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! #[documented_fields(trim = true, trim = false)]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
    //!
    //! ```compile_fail
    //! #[derive(documented::DocumentedFields)]
    //! struct NinetyNinePercentOf {
    //!     /// If you can call them people.
    //!     #[documented_fields(trim = true, trim = false)]
    //!     people: bool,
    //!     /// They're barking outside.
    //!     dogs: bool,
    //! }
    //! ```
}

#[cfg(feature = "customise")]
mod rename_on_container {
    //! ```
    //! use documented::DocumentedFields;
    //!
    //! #[derive(DocumentedFields)]
    //! enum Luigi {
    //!     /// Yes, his official last name is Mario.
    //!     Mario,
    //!     /// Did you know he has a Github account?
    //!     /// https://github.com/lnmangione
    //!     #[documented_fields(rename = "Who?")]
    //!     Mangione,
    //! }
    //!
    //! assert!(Luigi::get_field_docs("Mangione").is_err());
    //! assert!(Luigi::get_field_docs("Who?").is_ok());
    //! ```
    //!
    //! ```compile_fail
    //! use documented::DocumentedFields;
    //!
    //! #[derive(DocumentedFields)]
    //! #[documented_fields(rename = "Who?")]
    //! enum Luigi {
    //!     /// Yes, his official last name is Mario.
    //!     Mario,
    //!     /// Did you know he has a Github account?
    //!     /// https://github.com/lnmangione
    //!     Mangione,
    //! }
    //! ```
}
