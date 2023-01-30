use std::io;
use std::str::FromStr;

pub fn get_user_input<T>(prompt: &str) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
{
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let number: T = user_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    return number;
}