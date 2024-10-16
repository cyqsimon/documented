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

#[cfg(feature = "customise")]
mod test_customise {
    use documented::DocumentedVariantsOpt;

    #[test]
    fn default_works() {
        #[derive(DocumentedVariantsOpt)]
        #[documented_variants(default = Some("RIP"))]
        #[allow(dead_code)]
        enum Dead {
            Maggie,
            /// Maybe not?
            DotIO,
            #[documented_variants(default = Some("I think you're more prepared than Noah"))]
            Sean,
            // ah so here's a semi-reasonable use case for this
            #[documented_variants(default = None)]
            OJ,
        }

        assert_eq!(Dead::Maggie.get_variant_docs(), Some("RIP"));
        assert_eq!(Dead::DotIO.get_variant_docs(), Some("Maybe not?"));
        assert_eq!(
            Dead::Sean.get_variant_docs(),
            Some("I think you're more prepared than Noah")
        );
        assert_eq!(Dead::OJ.get_variant_docs(), None);
    }
}
