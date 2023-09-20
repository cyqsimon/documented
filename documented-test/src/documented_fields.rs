use documented::DocumentedFields;

#[test]
fn it_works() {
    #[derive(DocumentedFields)]
    #[allow(dead_code)]
    struct Foo {
        /// 1
        first: i32,
        second: i32,
        /// 3
        third: i32,
    }

    assert_eq!(Foo::FIELD_DOCS.len(), 3);
    assert_eq!(Foo::get_field_comment("first"), Some("1"));
    assert_eq!(Foo::get_field_comment("second"), Some(""));
    assert_eq!(Foo::get_field_comment("third"), Some("3"));
}
