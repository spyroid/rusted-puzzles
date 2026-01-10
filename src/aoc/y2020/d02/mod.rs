use fun_time::fun_time;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct PasswordRule {
    range: RangeInclusive<usize>,
    char: char,
    data: String,
}

#[fun_time(give_back)]
pub fn password_philosophy(input: Vec<String>) -> (usize, usize) {
    let rules = input
        .iter()
        .map(|line| line.parse::<PasswordRule>().unwrap())
        .collect::<Vec<_>>();

    let part1 = rules.iter().filter(|rule| rule.part1()).count();
    let part2 = rules.iter().filter(|rule| rule.part2()).count();

    (part1, part2)
}

impl PasswordRule {
    fn part1(&self) -> bool {
        let count = self.data.chars().filter(|&c| c == self.char).count();
        self.range.contains(&count)
    }
    fn part2(&self) -> bool {
        (self.data.chars().nth(*self.range.start() - 1) == Some(self.char))
            ^ (self.data.chars().nth(*self.range.end() - 1) == Some(self.char))
    }
}

impl FromStr for PasswordRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .split(|c| c == '-' || c == ' ' || c == ':')
            .filter(|s| !s.is_empty())
            .collect();

        if let [min, max, chr, data] = parts.as_slice() {
            Ok(PasswordRule {
                range: min.parse().unwrap()..=max.parse().unwrap(),
                char: chr.chars().next().ok_or("Invalid char")?,
                data: data.to_string(),
            })
        } else {
            Err("Format must be 'min-max c: data'".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::d02::password_philosophy;
    use crate::gears::{input_data_lines, print_debug};

    #[test]
    fn aoc() {
        print_debug(password_philosophy(input_data_lines()));
    }
}
