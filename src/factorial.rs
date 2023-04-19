#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;
use rayon::prelude::*;

fn number_to_vector(number: i32) -> Vec<BigInt> {
    (1..=number).into_par_iter().map(|i| BigInt::from(i)).collect()
}

pub fn factorial(number: i32) -> BigInt {
    number_to_vector(number).iter().fold(BigInt::from(1), |acc, x| acc * x)
}