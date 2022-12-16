extern crate num;
use num::BigInt;
use std::ops::Mul;

fn main() {
    let x = BigInt::parse_bytes(b"12345678901234567890", 10).unwrap();
    let y = BigInt::parse_bytes(b"9876543210987654321", 10).unwrap();

    let result = x.mul(y);

    println!("{}", result);  // Output: 12193263111263526400
}
