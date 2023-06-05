#![cfg(test)]

use documented::Documented;

/// 69
#[derive(Documented)]
struct Nice;

#[test]
fn it_works() {
    assert_eq!(Nice::DOCS, "69");
}
