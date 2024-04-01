use documented::{DocumentedVariants, Error};

#[test]
fn it_works() {
    #[derive(DocumentedVariants)]
    enum Bar {
        First,
        /// 2
        Second,
    }

    assert_eq!(
        Bar::First.get_variant_docs(),
        Err(Error::NoDocComments("First".into()))
    );
    assert_eq!(Bar::Second.get_variant_docs(), Ok("2"));
}
