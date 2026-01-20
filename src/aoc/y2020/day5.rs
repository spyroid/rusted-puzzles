use fun_time::fun_time;
use std::str::FromStr;

#[fun_time(give_back)]
pub fn binary_boarding(input: Vec<Box<str>>) -> (usize, usize) {
    let mut passes: Vec<_> = input
        .iter()
        .map(|line| line.parse::<BoardingPass>().unwrap().0)
        .collect();

    passes.sort_unstable();

    let part1 = *passes.last().unwrap();
    let part2 = passes.windows(2).find(|a| a[1] - a[0] > 1).unwrap()[0] + 1;

    (part1, part2)
}

struct BoardingPass(usize);

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(encoded: &str) -> Result<Self, Self::Err> {
        let a = encoded
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1");

        Ok(BoardingPass(usize::from_str_radix(&a, 2).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::day5::{BoardingPass, binary_boarding};
    use crate::gears::{input_data_lines, print_debug};

    #[test]
    fn aoc() {
        print_debug(binary_boarding(input_data_lines()));
    }

    #[test]
    fn valid() {
        let boarding = "FBFBBFFRLR".parse::<BoardingPass>().unwrap();
        assert_eq!(boarding.0, 357);
    }
}
