use documented::documented_function;



#[test]
fn it_works() {

    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn() {}

    assert_eq!(test_fn_docs, "This is a test function");

}


// Note: I found no way to test whether the visibility of the function is preserved.
// Manual testing showed that it is preserved, but I couldn't find a way to test it in code.


#[test]
fn parameters_work() {

    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn(_test: String) {}

    assert_eq!(test_fn_docs, "This is a test function");

}

#[test]
fn return_and_body_works() {

    /// This is a test function
    #[documented_function]
    #[allow(dead_code)]
    fn test_fn(input: String) -> String{
        input.to_ascii_lowercase()
    }

    assert_eq!(test_fn_docs, "This is a test function");

}