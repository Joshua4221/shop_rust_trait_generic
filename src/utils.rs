use std::{io, str::FromStr};

pub fn get_input<T>(prompt: &str) -> Result<T, String>
where
    T: FromStr,
{
    println!("{}", prompt);

    let mut input = String::new();

    while io::stdin().read_line(&mut input).is_err() {
        println!("the input you inputed is wrong");
    }

    input
        .trim()
        .parse::<T>()
        .map_err(|_| "Invalid input format".into())
}
