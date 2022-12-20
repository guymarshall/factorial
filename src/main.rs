mod user_input;

extern crate num;
use num::BigInt;
use std::ops::Mul;
use std::cmp::Ordering;

fn factorial(number: i32) -> BigInt {
    let mut factorial: BigInt = BigInt::parse_bytes(b"1", 10).unwrap();
    let mut counter: i32 = 1;

    while counter.cmp(&number) != Ordering::Greater {
        factorial = factorial.mul(counter);
        counter += 1;
    }

    factorial
}

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter number: ");

    println!("{}! = {}", user_input, factorial(user_input));
}
