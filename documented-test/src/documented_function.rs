use documented::documented_function;

#[test]
fn it_works() {
    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(TEST_FN_DOCS, "This is a test function");
}

// Note: I found no way to test whether the visibility of the function is preserved.
// Manual testing showed that it is preserved, but I couldn't find a way to test it in code.

#[test]
fn different_docs_work() {
    /// This is a test function
    /** This is the second line of the doc*/
    #[doc = "This is the third line of the doc"]
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(TEST_FN_DOCS, "This is a test function\nThis is the second line of the doc\nThis is the third line of the doc");
}

#[test]
fn parameters_work() {
    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn(_test: String) {}

    assert_eq!(TEST_FN_DOCS, "This is a test function");
}

#[test]
fn return_and_body_works() {
    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn(input: String) -> String {
        input.to_ascii_lowercase()
    }

    assert_eq!(TEST_FN_DOCS, "This is a test function");
}

#[test]
fn generics_and_lifetime_works() {
    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    #[allow(clippy::extra_unused_lifetimes)]
    fn test_fn<'a, T: std::fmt::Debug>(input: T) -> T {
        input
    }

    assert_eq!(TEST_FN_DOCS, "This is a test function");
}

#[test]
fn trim_works() {
    ///           This is a test function        
    ///        Test Trim
    #[documented_function(trim = true)] // technically redundant, as it's the default
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(TEST_FN_DOCS, "This is a test function\nTest Trim");
}

#[test]
fn no_trim_works() {
    ///           This is a test function        
    ///        Test Trim
    #[documented_function(trim = false)]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(
        TEST_FN_DOCS,
        "           This is a test function        \n        Test Trim"
    ); // The whitespace is preserved, even on the end of the first line
}
