use crate::aoc::y2020::d01::report_repair;
use std::fs;

mod aoc;
mod gears;

fn main() {
    let input = fs::read_to_string("./data/input.txt")
        .unwrap()
        .lines()
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("{:?}", report_repair(input));
}
