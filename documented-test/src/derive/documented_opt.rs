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
