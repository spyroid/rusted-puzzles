use crate::aoc::y2020::day19::Node::{Choice, Sequence, Terminal};
use fun_time::fun_time;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Terminal(char),
    Choice(Vec<usize>, Vec<usize>),
    Sequence(Vec<usize>),
}

type Rules = HashMap<usize, Node>;

#[fun_time(give_back)]
pub fn monster_messages(input: Vec<Box<str>>) -> (usize, usize) {
    let pos = input.iter().position(|s| s.is_empty()).unwrap();
    let i_rules = input[..pos].to_vec();
    let i_strings = input[pos + 1..].to_vec();

    let mut rules: Rules = i_rules
        .iter()
        .map(|s| {
            let parts = s.split(": ").collect::<Vec<_>>();
            let id = parts[0].parse::<usize>().unwrap();
            if parts[1].starts_with('"') {
                (id, Terminal(parts[1].chars().nth(1).unwrap()))
            } else {
                let all_str = parts[1].split_whitespace().collect::<Vec<_>>();
                let pos = all_str.iter().position(|s| *s == "|");
                let all = all_str
                    .iter()
                    .filter(|s| **s != "|")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                match pos {
                    Some(p) => (id, Choice(all[..p].to_vec(), all[p..].to_vec())),
                    None => (id, Sequence(all.to_vec())),
                }
            }
        })
        .collect();

    let do_count = |rules: &Rules| -> usize {
        let regexp = format!("^{}$", to_regex_str(&rules, 0, 25));
        let re = Regex::new(&regexp).unwrap();
        i_strings.iter().filter(|s| re.is_match(s)).count()
    };

    let part1 = do_count(&rules);

    rules.insert(8, Choice(vec![42], vec![42, 8]));
    rules.insert(11, Choice(vec![42, 31], vec![42, 11, 31]));

    let part2 = do_count(&rules);

    (part1, part2)
}

fn to_regex_str(rules: &Rules, idx: usize, depth: usize) -> String {
    if depth == 0 {
        return String::new();
    }
    let rule = rules.get(&idx).unwrap();
    match rule {
        Terminal(c) => format!("{}", c),
        Sequence(parts) => {
            let seq = parts
                .iter()
                .map(|&sub_id| to_regex_str(rules, sub_id, depth - 1))
                .collect::<String>();
            format!("({seq})")
        }
        Choice(l1, l2) => format!(
            "({}|{})",
            l1.iter()
                .map(|i| to_regex_str(rules, *i, depth - 1))
                .collect::<String>(),
            l2.iter()
                .map(|i| to_regex_str(rules, *i, depth - 1))
                .collect::<String>()
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::day19::monster_messages;
    use crate::gears::{input_data_lines, print_result};

    #[test]
    fn aoc() {
        print_result(monster_messages(input_data_lines()));
    }
}
