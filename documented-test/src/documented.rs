mod test_use {
    use documented::Documented;

    #[test]
    fn it_works() {
        /// 69
        #[derive(Documented)]
        struct Nice;

        assert_eq!(Nice::DOCS, "69");
    }

    #[test]
    fn multi_line_works() {
        /// 69
        /// 420
        ///
        /// 1337
        #[derive(Documented)]
        struct Nice;

        let docs = "69\n420\n\n1337";
        assert_eq!(Nice::DOCS, docs);
    }

    #[test]
    fn every_style_works() {
        /// 69
        /** Very nice
        420 */
        #[doc = "1337"]
        #[derive(Documented)]
        struct Nicer;

        let docs = "69\nVery nice\n420\n1337";
        assert_eq!(Nicer::DOCS, docs);
    }

    #[test]
    fn readme_example_works() {
        /// Nice.
        /// Multiple single-line doc comments are supported.
        ///
        /** Multi-line doc comments are supported too.
        Each line of the multi-line block is individually trimmed.
        Note the lack of spaces in front of this line.
        */
        #[doc = "Attribute-style documentation is supported too."]
        #[derive(Documented)]
        struct BornIn69;

        let doc_str = "Nice.
Multiple single-line doc comments are supported.

Multi-line doc comments are supported too.
Each line of the multi-line block is individually trimmed.
Note the lack of spaces in front of this line.

Attribute-style documentation is supported too.";
        assert_eq!(BornIn69::DOCS, doc_str);
    }

    #[test]
    fn generic_type_works() {
        /// Wow
        #[allow(dead_code)]
        #[derive(Documented)]
        struct Doge<T> {
            much: T,
        }

        assert_eq!(Doge::<u8>::DOCS, "Wow");
    }

    #[test]
    fn generic_type_with_bounds_works() {
        /// Wow
        #[allow(dead_code)]
        #[derive(Documented)]
        struct Doge<T: Copy> {
            much: T,
        }

        assert_eq!(Doge::<u8>::DOCS, "Wow");
    }

    #[test]
    fn const_generic_type_works() {
        /// Wow
        #[allow(dead_code)]
        #[derive(Documented)]
        struct Doge<const LEN: usize> {
            much: [u8; LEN],
        }

        assert_eq!(Doge::<69>::DOCS, "Wow");
    }

    #[test]
    fn lifetimed_type_works() {
        /// Wow
        #[allow(dead_code)]
        #[derive(Documented)]
        struct Doge<'a> {
            much: &'a str,
        }

        assert_eq!(Doge::DOCS, "Wow");
    }
}

mod test_qualified {
    #[test]
    fn it_works() {
        /// 69
        #[derive(documented::Documented)]
        struct Nice;

        assert_eq!(<Nice as documented::Documented>::DOCS, "69");
    }
}
