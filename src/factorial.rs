#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;
use rayon::prelude::*;

fn number_to_vector(number: i32) -> Vec<BigInt> {
    (1..=number).into_par_iter().map(BigInt::from).collect()
}

pub fn factorial(number: i32) -> BigInt {
    number_to_vector(number)
        .iter()
        .fold(BigInt::from(1), |accumulator, current| {
            accumulator * current
        })
}

pub fn format_scientific(number: &BigInt) -> String {
    let digit_count: i64 = number.to_string().len() as i64;
    let exponent: i64 = digit_count - 1;
    let mantissa: BigInt = number / BigInt::from(10).pow(exponent as u32);
    format!("{}x10^{}", mantissa, exponent)
}
