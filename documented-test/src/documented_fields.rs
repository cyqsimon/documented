use documented::{DocumentedFields, Error};

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
    assert_eq!(Foo::get_field_comment("first"), Ok("1"));
    assert_eq!(
        Foo::get_field_comment("second"),
        Err(Error::NoDocComments("second".into()))
    );
    assert_eq!(Foo::get_field_comment("third"), Ok("3"));
    assert_eq!(
        Foo::get_field_comment("fourth"),
        Err(Error::NoSuchField("fourth".into()))
    );
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
    assert_eq!(
        Bar::get_field_comment("First"),
        Err(Error::NoDocComments("First".into()))
    );
    assert_eq!(Bar::get_field_comment("Second"), Ok("2"));
    assert_eq!(
        Bar::get_field_comment("Third"),
        Err(Error::NoSuchField("Third".into()))
    );
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
    assert_eq!(
        FooBar::get_field_comment("first"),
        Err(Error::NoDocComments("first".into()))
    );
    assert_eq!(FooBar::get_field_comment("second"), Ok("2"));
    assert_eq!(
        FooBar::get_field_comment("third"),
        Err(Error::NoDocComments("third".into()))
    );
}

#[test]
fn unnamed_fields() {
    #[derive(DocumentedFields)]
    #[allow(dead_code)]
    struct Foo(
        /// 0
        i32,
        /// 1
        u32,
        i64,
    );

    assert_eq!(Foo::FIELD_DOCS.len(), 3);
    assert_eq!(Foo::FIELD_DOCS[0], Some("0"));
    assert_eq!(Foo::FIELD_DOCS[1], Some("1"));
    assert_eq!(Foo::FIELD_DOCS[2], None);
}
