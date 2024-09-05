use documented::docs_const;

#[test]
fn it_works() {
    /// This is a test function
    #[docs_const]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(TEST_FN_DOCS, "This is a test function");
}

// Note: I found no way to test whether the visibility of the item is preserved.
// Manual testing showed that it is preserved, but I couldn't find a way to test it in code.

#[test]
fn multiple_docs_work() {
    /// This is a test function
    /** This is the second line of the doc*/
    #[doc = "This is the third line of the doc"]
    #[docs_const]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(TEST_FN_DOCS, "This is a test function\nThis is the second line of the doc\nThis is the third line of the doc");
}

#[cfg(feature = "customise")]
mod test_customise {
    use documented::docs_const;

    #[test]
    fn custom_visibility_works() {
        mod class {
            use documented::docs_const;

            #[docs_const(vis = pub)]
            #[allow(dead_code)]
            /// Arjun!
            trait RandomStudent {}
        }

        assert_eq!(class::RANDOMSTUDENT_DOCS, "Arjun!");
    }

    #[test]
    fn rename_works() {
        /// Suspicious
        #[docs_const(name = "NEVER_PLAY_F6")]
        #[allow(dead_code)]
        mod f6 {}

        assert_eq!(NEVER_PLAY_F6, "Suspicious");
    }

    #[test]
    fn trim_works() {
        ///           This is a test function        
        ///        Test Trim
        #[docs_const(trim = true)] // technically redundant, as it's the default
        #[allow(dead_code)]
        fn test_fn() {}

        assert_eq!(TEST_FN_DOCS, "This is a test function\nTest Trim");
    }

    #[test]
    fn no_trim_works() {
        ///           This is a test function        
        ///        Test Trim
        #[docs_const(trim = false)]
        #[allow(dead_code)]
        fn test_fn() {}

        assert_eq!(
            TEST_FN_DOCS,
            "           This is a test function        \n        Test Trim"
        ); // The whitespace is preserved, even on the end of the first line
    }
}
