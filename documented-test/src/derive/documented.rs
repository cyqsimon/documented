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

#[cfg(feature = "customise")]
mod test_customise {
    use documented::Documented;

    #[test]
    fn empty_customise_works() {
        /** Wow
            much
            doge
        */
        #[derive(Documented)]
        #[documented()]
        struct Doge;

        let doc_str = "Wow
much
doge
";
        assert_eq!(Doge::DOCS, doc_str);
    }

    #[test]
    fn multiple_attrs_works() {
        /** Wow
            much
            doge
        */
        #[derive(Documented)]
        #[documented()]
        #[documented()]
        struct Doge;

        let doc_str = "Wow
much
doge
";
        assert_eq!(Doge::DOCS, doc_str);
    }

    #[test]
    fn default_works_with_literal() {
        #[derive(Documented)]
        #[documented(default = "3 goals 2 assists!")]
        struct Age37;

        assert_eq!(Age37::DOCS, "3 goals 2 assists!");
    }

    #[test]
    fn default_works_with_const() {
        const DOC_STR: &str = "3 goals 2 assists!";

        #[derive(Documented)]
        #[documented(default = DOC_STR)]
        struct Age37;

        assert_eq!(Age37::DOCS, DOC_STR);
    }

    #[test]
    fn default_works_with_macros() {
        #[derive(Documented)]
        #[documented(default = concat!("3 goals ", "2 assists!"))]
        struct Age37;

        assert_eq!(Age37::DOCS, "3 goals 2 assists!");
    }

    #[test]
    fn trim_false_works() {
        /** Wow
            much
            doge
        */
        #[derive(Documented)]
        #[documented(trim = false)]
        struct Doge;

        let doc_str = " Wow
            much
            doge
        ";
        assert_eq!(Doge::DOCS, doc_str);
    }
}
