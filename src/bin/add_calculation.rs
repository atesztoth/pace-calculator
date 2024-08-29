use self::models::*;
use diesel::prelude::*;
use pace_calculator::*;
use std::fmt::Debug;
use std::io::stdin;
use std::str::FromStr;

pub fn main() {
    let connection = &mut establish_connection();

    println!("Write your time in seconds!");
    let time: i32 = parse_number("Failed to parse input_time as i32");

    println!("Write your distance in meters!");
    let distance: i32 = parse_number("Failed to parse distance as i32");

    println!("Write your pace in seconds!");
    let pace: i32 = parse_number("Failed to parse pace as i32");

    let calculation = create_calculation(connection, time, distance, pace);

    println!("Stored calculation: {:?}", calculation);
}

fn parse_number<F: FromStr>(fail_message: &str) -> F
where
    <F as FromStr>::Err: Debug,
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().parse().expect(fail_message)
}
