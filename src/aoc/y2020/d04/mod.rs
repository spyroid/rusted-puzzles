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
                .map(|(k, v)| (k, &v[1..])) // Skip the colon
                .collect();
            Passport { fields }
        })
        .collect();

    let part1 = passports.iter().filter(|p| p.valid()).count();

    (part1, 0)
}

#[derive(Debug)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    pub fn valid(&self) -> bool {
        self.fields.len() == 8 || (self.fields.len() == 7 && !self.fields.contains_key("cid"))
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
