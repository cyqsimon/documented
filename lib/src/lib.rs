#[cfg(test)]
mod test {
    pub use documented_derive::Documented;

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
}
