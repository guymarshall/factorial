extern crate num;
use num::BigInt;
use std::ops::Mul;
use std::cmp::Ordering;

pub fn factorial_single_threaded(number: i32) -> BigInt {
    let mut factorial: BigInt = BigInt::parse_bytes(b"1", 10).unwrap();
    let mut counter: i32 = 1;

    while counter.cmp(&number) != Ordering::Greater {
        factorial = factorial.mul(counter);
        counter += 1;
    }

    factorial
}

fn number_to_vector(number: i32) -> Vec<BigInt> {
    let mut numbers: Vec<BigInt> = Vec::new();

    (1..=number).into_iter().for_each(|i| {
        numbers.push(BigInt::from(i));
    });

    numbers
}

pub fn factorial(number: i32) -> BigInt {
    let numbers: Vec<BigInt> = number_to_vector(number);

    let factorial: BigInt = numbers.iter().fold(BigInt::from(1), |acc, x| acc * x);

    factorial
}