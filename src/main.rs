mod user_input;
mod factorial;

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter number: ");

    if user_input < 0 {
        println!("Factorial is not defined for negative numbers");
        return;
    }

    if user_input == 0 {
        println!("0! = 1");
        return;
    }

    println!("{}! = {}", user_input, factorial::factorial(user_input));
}
