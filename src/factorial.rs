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
    // elements.iter_mut().for_each(|x| *x *= c);
    let mut factorial: BigInt = BigInt::from(1);

    for number in numbers {
        factorial = factorial.mul(number);
    }
    factorial
}