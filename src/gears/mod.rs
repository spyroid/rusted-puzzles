use std::fmt::Debug;
use std::fs;
use std::time::Duration;

pub mod point;

pub fn input_data_string() -> Box<String> {
    Box::from(fs::read_to_string("./data/input.txt").unwrap())
}

pub fn input_data_lines() -> Vec<Box<str>> {
    input_data_string().lines().map(|s| Box::from(s)).collect()
}

pub fn input_data_u32s() -> Vec<u32> {
    input_data_lines()
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn print_result<T: Debug>(value: (T, Duration)) {
    println!("\n{:?} -> {:?}", value.0, value.1);
}
