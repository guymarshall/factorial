extern crate num;
use num::BigInt;
use std::ops::Mul;
use std::cmp::Ordering;

pub fn factorial(number: i32) -> BigInt {
    let mut factorial: BigInt = BigInt::parse_bytes(b"1", 10).unwrap();
    let mut counter: i32 = 1;

    while counter.cmp(&number) != Ordering::Greater {
        factorial = factorial.mul(counter);
        counter += 1;
    }

    factorial
}