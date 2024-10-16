use documented::DocumentedOpt;

#[test]
fn some_works() {
    /// 69
    #[derive(DocumentedOpt)]
    struct Nice;

    assert_eq!(Nice::DOCS, Some("69"));
}

#[test]
fn none_works() {
    #[derive(DocumentedOpt)]
    struct NotSoNice;

    assert_eq!(NotSoNice::DOCS, None);
}

#[cfg(feature = "customise")]
mod test_customise {
    use documented::DocumentedOpt;

    #[test]
    fn default_works() {
        #[derive(DocumentedOpt)]
        #[documented(default = Some("Nice catch!"))]
        struct NiceFlight;

        assert_eq!(NiceFlight::DOCS, Some("Nice catch!"));
    }
}
