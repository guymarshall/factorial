extern crate num;

use std::time::Instant;

use bigdecimal::BigDecimal;
use num::BigInt;
use promptput::input;
use rayon::prelude::*;

fn format_scientific(number: BigInt) -> String {
    if number < BigInt::from(1000) {
        return number.to_string();
    }

    let number_decimal: BigDecimal = BigDecimal::from(number);
    format!("{number_decimal:.2E}")
}

fn calculate_factorial(number: i32) -> BigInt {
    (2..=number)
        .into_par_iter()
        .map(BigInt::from)
        .reduce_with(|a: BigInt, b: BigInt| a * b)
        .unwrap_or(BigInt::from(1))
}

fn main() {
    let number: i32 = input("Enter number:");

    let start_time: Instant = Instant::now();
    let factorial: BigInt = calculate_factorial(number);

    println!("Formatting...");
    let factorial_formatted: String = format_scientific(factorial);
    println!("{number}! = {factorial_formatted}");

    let end_time: Instant = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);
}
