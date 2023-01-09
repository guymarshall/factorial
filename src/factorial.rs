extern crate num;
use num::BigInt;
use rayon::prelude::*;

fn number_to_vector(number: i32) -> Vec<BigInt> {
    (1..=number).into_par_iter().map(|i| BigInt::from(i)).collect()
}

pub fn factorial(number: i32) -> BigInt {
    if number < 0 {
        panic!("Factorial is not defined for negative numbers");
    }

    if number == 0 {
        return BigInt::from(1);
    }
    
    let numbers: Vec<BigInt> = number_to_vector(number);

    let factorial: BigInt = numbers.iter().fold(BigInt::from(1), |acc, x| acc * x);

    factorial
}