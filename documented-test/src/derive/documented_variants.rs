use documented::DocumentedVariants;

#[test]
fn it_works() {
    #[derive(DocumentedVariants)]
    enum Foo {
        /// 1
        First,
        /// 2
        Second,
    }

    assert_eq!(Foo::First.get_variant_docs(), "1");
    assert_eq!(Foo::Second.get_variant_docs(), "2");
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

    assert_eq!(Bar::Unit.get_variant_docs(), "A unit variant.");
    assert_eq!(Bar::Tuple0().get_variant_docs(), "A 0-tuple variant.");
    assert_eq!(Bar::Tuple1(1).get_variant_docs(), "A 1-tuple variant.");
    assert_eq!(Bar::Tuple2(2, 2).get_variant_docs(), "A 2-tuple variant.");
    assert_eq!(
        Bar::Struct { alpha: 0, bravo: 0 }.get_variant_docs(),
        "A struct variant."
    );
    assert_eq!(
        Bar::StructEmpty {}.get_variant_docs(),
        "An empty struct variant."
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

    assert_eq!(Foo::<u8, u8>::Rufus(69).get_variant_docs(), "600");
    assert_eq!(Foo::Dufus(69, 420).get_variant_docs(), "599");
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

    assert_eq!(Foo::<u8, u8>::Rufus(69).get_variant_docs(), "600");
    assert_eq!(Foo::Dufus(69, 420).get_variant_docs(), "599");
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

    assert_eq!(Foo::Rufus([42; 69]).get_variant_docs(), "600");
    assert_eq!(Foo::Dufus([42; 69]).get_variant_docs(), "599");
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

    assert_eq!(Foo::Rufus(&69).get_variant_docs(), "600");
    assert_eq!(Foo::Dufus(69, &420).get_variant_docs(), "599");
}

#[cfg(feature = "customise")]
mod test_customise {
    use documented::DocumentedVariants;

    #[test]
    fn empty_customise_works() {
        #[derive(DocumentedVariants)]
        #[documented_variants()]
        #[allow(dead_code)]
        enum Name {
            /// Wow
            Doge,
            /// RIP
            Kabuso,
        }

        assert_eq!(Name::Doge.get_variant_docs(), "Wow");
        assert_eq!(Name::Kabuso.get_variant_docs(), "RIP");
    }

    #[test]
    fn multiple_attrs_works() {
        #[derive(DocumentedVariants)]
        #[documented_variants()]
        #[documented_variants()]
        #[allow(dead_code)]
        enum Name {
            /// Wow
            #[documented_variants()]
            #[documented_variants()]
            Doge,
            /// RIP
            Kabuso,
        }

        assert_eq!(Name::Doge.get_variant_docs(), "Wow");
        assert_eq!(Name::Kabuso.get_variant_docs(), "RIP");
    }

    #[test]
    fn container_customise_works() {
        #[derive(DocumentedVariants)]
        #[documented_variants(trim = false)]
        #[allow(dead_code)]
        enum Name {
            ///     Wow
            Doge,
            ///     RIP
            Kabuso,
        }

        assert_eq!(Name::Doge.get_variant_docs(), "     Wow");
        assert_eq!(Name::Kabuso.get_variant_docs(), "     RIP");
    }

    #[test]
    fn field_customise_works() {
        #[derive(DocumentedVariants)]
        #[allow(dead_code)]
        enum Name {
            ///     Wow
            #[documented_variants(trim = false)]
            Doge,
            ///     RIP
            Kabuso,
        }

        assert_eq!(Name::Doge.get_variant_docs(), "     Wow");
        assert_eq!(Name::Kabuso.get_variant_docs(), "RIP");
    }

    #[test]
    fn field_customise_override_works() {
        #[derive(DocumentedVariants)]
        #[documented_variants(trim = false)]
        #[allow(dead_code)]
        enum Name {
            ///     Wow
            #[documented_variants(trim = true)]
            Doge,
            ///     RIP
            Kabuso,
        }

        assert_eq!(Name::Doge.get_variant_docs(), "Wow");
        assert_eq!(Name::Kabuso.get_variant_docs(), "     RIP");
    }

    #[test]
    fn default_works() {
        #[derive(DocumentedVariants)]
        #[documented_variants(default = "RIP")]
        #[allow(dead_code)]
        enum Dead {
            Maggie,
            /// Maybe not yet?
            DotIO,
            // don't know why anyone would want to do this but it's supported
            #[documented_variants(default = "I think you're more prepared than Noah")]
            Sean,
        }

        assert_eq!(Dead::Maggie.get_variant_docs(), "RIP");
        assert_eq!(Dead::DotIO.get_variant_docs(), "Maybe not yet?");
        assert_eq!(
            Dead::Sean.get_variant_docs(),
            "I think you're more prepared than Noah"
        );
    }
}
