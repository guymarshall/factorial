#![forbid(unsafe_code)]

use std::process::exit;
use num::BigInt;

mod factorial;
mod user_input;

fn main() {
    let user_input: i32 = user_input::input("Enter number: ");

    if user_input < 0 {
        println!("Factorial is not defined for negative numbers");
        exit(1);
    }

    if user_input == 0 {
        println!("0! = 1");
        exit(1);
    }

    let factorial: BigInt = factorial::factorial(user_input);
    let formatted_factorial: String = factorial::format_scientific(&factorial);

    println!("{}! = {}", user_input, formatted_factorial);
}
