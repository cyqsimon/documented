use documented::{DocumentedFieldsOpt, Error};

#[test]
fn it_works() {
    #[derive(DocumentedFieldsOpt)]
    #[allow(dead_code)]
    struct Foo {
        /// 1
        first: i32,
        second: i32,
        /// 3
        third: i32,
    }

    assert_eq!(Foo::FIELD_NAMES, vec!["first", "second", "third"]);
    assert_eq!(Foo::FIELD_DOCS.len(), 3);
    assert_eq!(Foo::get_field_docs("first"), Ok("1"));
    assert_eq!(
        Foo::get_field_docs("second"),
        Err(Error::NoDocComments("second".into()))
    );
    assert_eq!(Foo::get_field_docs("third"), Ok("3"));
    assert_eq!(
        Foo::get_field_docs("fourth"),
        Err(Error::NoSuchField("fourth".into()))
    );
}

#[test]
fn enum_works() {
    #[derive(DocumentedFieldsOpt)]
    #[allow(dead_code)]
    enum Bar {
        First,
        /// 2
        Second,
    }

    assert_eq!(Bar::FIELD_NAMES, vec!["First", "Second"]);
    assert_eq!(Bar::FIELD_DOCS.len(), 2);
    assert_eq!(
        Bar::get_field_docs("First"),
        Err(Error::NoDocComments("First".into()))
    );
    assert_eq!(Bar::get_field_docs("Second"), Ok("2"));
    assert_eq!(
        Bar::get_field_docs("Third"),
        Err(Error::NoSuchField("Third".into()))
    );
}

#[test]
fn union_works() {
    #[derive(DocumentedFieldsOpt)]
    #[allow(dead_code)]
    union FooBar {
        first: i32,
        /// 2
        second: i32,
        third: i32,
    }

    assert_eq!(FooBar::FIELD_NAMES, vec!["first", "second", "third"]);
    assert_eq!(FooBar::FIELD_DOCS.len(), 3);
    assert_eq!(
        FooBar::get_field_docs("first"),
        Err(Error::NoDocComments("first".into()))
    );
    assert_eq!(FooBar::get_field_docs("second"), Ok("2"));
    assert_eq!(
        FooBar::get_field_docs("third"),
        Err(Error::NoDocComments("third".into()))
    );
}

#[cfg(feature = "customise")]
mod test_customise {
    use documented::{DocumentedFieldsOpt, Error};

    #[test]
    fn default_works() {
        #[derive(DocumentedFieldsOpt)]
        #[documented_fields(default = Some("Woosh"))]
        #[allow(dead_code)]
        enum Mission {
            /// Rumble
            Launch,
            Boost,
            #[documented_fields(default = None)]
            FreeFall,
            #[documented_fields(default = Some("Boom"))]
            Touchdown,
        }

        assert_eq!(Mission::FIELD_NAMES, vec!["Launch", "Boost", "FreeFall", "Touchdown"]);
        assert_eq!(Mission::get_field_docs("Launch"), Ok("Rumble"));
        assert_eq!(Mission::get_field_docs("Boost"), Ok("Woosh"));
        assert_eq!(
            Mission::get_field_docs("FreeFall"),
            Err(Error::NoDocComments("FreeFall".into()))
        );
        assert_eq!(Mission::get_field_docs("Touchdown"), Ok("Boom"));
    }
}
