use fun_time::fun_time;
use itertools::Itertools;
use std::str::FromStr;

#[fun_time(give_back)]
fn factory(input: Vec<Box<str>>) -> usize {
    let machines: Vec<Machine> = input.iter().map(|x| x.parse().unwrap()).collect();
    let part1: usize = machines
        .iter()
        .map(|m| {
            // println!("{:?}", m);
            let variants = (1..=m.buttons.len())
                .flat_map(|size| m.buttons.iter().combinations(size))
                .find(|vec| {
                    let n = vec.iter().fold(0, |acc, &x| acc ^ x);
                    n == m.lights
                });
            variants.unwrap().len()
        })
        .sum();

    part1
}

#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Machine, String> {
        let parts: Vec<&str> = s
            .split(|c| {
                c == ' ' || c == '[' || c == ']' || c == '(' || c == ')' || c == '{' || c == '}'
            })
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Machine {
            lights: u16::from_str_radix(
                parts[0]
                    .chars()
                    .rev()
                    .collect::<String>()
                    .replace(".", "0")
                    .replace("#", "1")
                    .as_str(),
                2,
            )
            .unwrap(),
            buttons: parts[1..parts.len() - 1]
                .iter()
                .map(|p| {
                    let n = p
                        .split(',')
                        .map(|s| s.parse::<u16>().unwrap())
                        .fold(0u16, |acc, x| acc | 1 << x);
                    n
                })
                .collect(),
            joltages: parts
                .last()
                .unwrap()
                .split(',')
                .filter_map(|n| n.parse().ok())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2025::day10::factory;
    use crate::gears::{input_data_lines, print_result};

    #[test]
    fn aoc() {
        print_result(factory(input_data_lines()));
    }
}
