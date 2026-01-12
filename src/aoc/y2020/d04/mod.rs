use fun_time::fun_time;
use std::collections::HashMap;

#[fun_time(give_back)]
pub fn passport_processing(input: Box<String>) -> (usize, usize) {
    let passports: Vec<_> = input
        .split("\n\n")
        .map(|line| {
            let fields: HashMap<_, _> = line
                .split_whitespace()
                .map(|s| s.split_at(s.find(':').unwrap_or(0)))
                .filter(|(k, _v)| *k != "cid")
                .map(|(k, v)| (k, &v[1..]))
                .collect();
            Passport { fields }
        })
        .collect();

    let part1 = passports.iter().filter(|p| p.valid()).count();
    let part2 = passports.iter().filter(|p| p.strict_valid()).count();

    (part1, part2)
}

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    pub fn valid(&self) -> bool {
        self.fields.len() == 7
    }
    fn validate_range(s: &str, min: u32, max: u32) -> bool {
        s.parse::<u32>()
            .map_or(false, |num| (min..=max).contains(&num))
    }

    const COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    fn validate_field(field: &str, value: &str) -> bool {
        match field {
            "byr" => Self::validate_range(value, 1920, 2002),
            "iyr" => Self::validate_range(value, 2010, 2020),
            "eyr" => Self::validate_range(value, 2020, 2030),
            "ecl" => Self::COLORS.contains(&value),
            "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
            "hcl" => {
                value.len() == 7
                    && value.starts_with('#')
                    && value[1..].chars().all(|c| c.is_ascii_hexdigit())
            }
            "hgt" => match value.split_at(value.len().saturating_sub(2)) {
                (n, "cm") => n.parse::<u32>().map_or(false, |h| (150..=193).contains(&h)),
                (n, "in") => n.parse::<u32>().map_or(false, |h| (59..=76).contains(&h)),
                _ => false,
            },
            _ => false,
        }
    }
    pub fn strict_valid(&self) -> bool {
        self.valid()
            && self
                .fields
                .iter()
                .all(|(&k, &v)| Self::validate_field(k, v))
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::d04::passport_processing;
    use crate::gears::{input_data_string, print_debug};

    #[test]
    fn aoc() {
        print_debug(passport_processing(input_data_string()));
    }
}
