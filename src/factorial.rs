extern crate num;
use num::BigInt;

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