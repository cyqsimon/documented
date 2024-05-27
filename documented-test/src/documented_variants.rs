use documented::{DocumentedVariants, Error};

#[test]
fn it_works() {
    #[derive(DocumentedVariants)]
    enum Foo {
        First,
        /// 2
        Second,
    }

    assert_eq!(
        Foo::First.get_variant_docs(),
        Err(Error::NoDocComments("First".into()))
    );
    assert_eq!(Foo::Second.get_variant_docs(), Ok("2"));
}

#[test]
fn works_on_adt_enums() {
    #[allow(dead_code)]
    #[derive(DocumentedVariants)]
    enum Bar {
        /// A unit variant.
        Unit,
        /// A 0-tuple variant.
        Tuple0(),
        /// A 1-tuple variant.
        Tuple1(u8),
        /// A 2-tuple variant.
        Tuple2(u8, u16),
        /// A struct variant.
        Struct { alpha: u8, bravo: u16 },
        /// An empty struct variant.
        StructEmpty {},
    }

    assert_eq!(Bar::Unit.get_variant_docs(), Ok("A unit variant."));
    assert_eq!(Bar::Tuple0().get_variant_docs(), Ok("A 0-tuple variant."));
    assert_eq!(Bar::Tuple1(1).get_variant_docs(), Ok("A 1-tuple variant."));
    assert_eq!(
        Bar::Tuple2(2, 2).get_variant_docs(),
        Ok("A 2-tuple variant.")
    );
    assert_eq!(
        Bar::Struct { alpha: 0, bravo: 0 }.get_variant_docs(),
        Ok("A struct variant.")
    );
    assert_eq!(
        Bar::StructEmpty {}.get_variant_docs(),
        Ok("An empty struct variant.")
    );
}

#[test]
fn works_on_generic_enums() {
    #[allow(dead_code)]
    #[derive(DocumentedVariants)]
    enum Foo<T, U> {
        /// 600
        Rufus(T),
        /// 599
        Dufus(T, U),
    }

    assert_eq!(Foo::<u8, u8>::Rufus(69).get_variant_docs(), Ok("600"));
    assert_eq!(Foo::Dufus(69, 420).get_variant_docs(), Ok("599"));
}

#[test]
fn works_on_generic_enums_with_bounds() {
    #[allow(dead_code)]
    #[derive(DocumentedVariants)]
    enum Foo<T: Copy, U: std::fmt::Debug> {
        /// 600
        Rufus(T),
        /// 599
        Dufus(T, U),
    }

    assert_eq!(Foo::<u8, u8>::Rufus(69).get_variant_docs(), Ok("600"));
    assert_eq!(Foo::Dufus(69, 420).get_variant_docs(), Ok("599"));
}

#[test]
fn works_on_const_generic_enums() {
    #[allow(dead_code)]
    #[derive(DocumentedVariants)]
    enum Foo<const LEN: usize> {
        /// 600
        Rufus([u8; LEN]),
        /// 599
        Dufus([i8; LEN]),
    }

    assert_eq!(Foo::Rufus([42; 69]).get_variant_docs(), Ok("600"));
    assert_eq!(Foo::Dufus([42; 69]).get_variant_docs(), Ok("599"));
}

#[test]
fn works_on_lifetimed_enums() {
    #[allow(dead_code)]
    #[derive(DocumentedVariants)]
    enum Foo<'a, T> {
        /// 600
        Rufus(&'a T),
        /// 599
        Dufus(T, &'a T),
    }

    assert_eq!(Foo::Rufus(&69).get_variant_docs(), Ok("600"));
    assert_eq!(Foo::Dufus(69, &420).get_variant_docs(), Ok("599"));
}
