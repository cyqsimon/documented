use documented::DocumentedVariantsOpt;

#[test]
fn it_works() {
    #[derive(DocumentedVariantsOpt)]
    enum Foo {
        First,
        /// 2
        Second,
    }

    assert_eq!(Foo::First.get_variant_docs(), None);
    assert_eq!(Foo::Second.get_variant_docs(), Some("2"));
}
