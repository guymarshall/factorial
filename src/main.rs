mod user_input;
mod factorial;

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter number: ");

    println!("{}! = {}", user_input, factorial::factorial(user_input));
}
