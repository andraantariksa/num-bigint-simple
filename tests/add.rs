extern crate num_bigint_simple;

use num_bigint_simple::bigint::*;

#[test]
fn two_plus_two() {
    assert_eq!(
        BigUInteger::from_str("4"),
        BigUInteger::from_str("2") + BigUInteger::from_str("2")
    );
}

#[test]
fn two_sub_two() {
    assert_eq!(
        BigUInteger::from_str("0"),
        BigUInteger::from_str("2") - BigUInteger::from_str("2")
    );
}

#[test]
fn four_sub_two() {
    assert_eq!(
        BigUInteger::from_str("2"),
        BigUInteger::from_str("4") - BigUInteger::from_str("2")
    );
}
