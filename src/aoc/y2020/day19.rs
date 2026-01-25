use fun_time::fun_time;
use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Terminal(char),
    Choice((usize, usize), (usize, usize)),
    Sequence(Vec<usize>),
}

#[fun_time(give_back)]
pub fn monster_messages(input: Vec<Box<str>>) -> (usize, usize) {
    let rules: HashMap<usize, Node> = input
        .iter()
        .map(|s| {
            let parts = s.split(": ").collect::<Vec<_>>();
            let id = parts[0].parse::<usize>().unwrap();
            if parts[1].starts_with('"') {
                (id, Node::Terminal(parts[1].chars().nth(1).unwrap()))
            } else {
                let v = parts[1]
                    .replace("|", "")
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                if parts[1].contains("|") {
                    (id, Node::Choice((v[0], v[1]), (v[2], v[3])))
                } else {
                    (id, Node::Sequence(v))
                }
            }
        })
        .collect();

    println!("{:?}", rules);

    (0, 0)
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::day19::monster_messages;
    use crate::gears::{input_data_lines, print_debug};

    #[test]
    fn aoc() {
        print_debug(monster_messages(input_data_lines()));
    }
}
