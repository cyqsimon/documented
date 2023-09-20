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
    assert_eq!(Foo::FIELD_DOCS[1], "");
}

#[test]
fn enum_works() {
    #[derive(DocumentedFields)]
    #[allow(dead_code)]
    enum Bar {
        First,
        /// 2
        Second,
    }

    assert_eq!(Bar::FIELD_DOCS.len(), 2);
    assert_eq!(Bar::FIELD_DOCS[0], "");
    assert_eq!(Bar::get_field_comment("Second"), Some("2"))
}

#[test]
fn union_works() {
    #[derive(DocumentedFields)]
    #[allow(dead_code)]
    union FooBar {
        first: i32,
        /// 2
        second: i32,
        third: i32,
    }

    assert_eq!(FooBar::FIELD_DOCS.len(), 3);
    assert_eq!(FooBar::get_field_comment("first"), Some(""));
    assert_eq!(FooBar::get_field_comment("second"), Some("2"));
    assert_eq!(FooBar::get_field_comment("third"), Some(""));
}

#[test]
fn unnamed_fields() {
    #[derive(DocumentedFields)]
    #[allow(dead_code)]
    struct Foo(
        /// 0
        i32,
        /// 1
        u32
    );

    assert_eq!(Foo::FIELD_DOCS.len(), 2);
    assert_eq!(Foo::FIELD_DOCS[0], "0");
    assert_eq!(Foo::FIELD_DOCS[1], "1");
}


