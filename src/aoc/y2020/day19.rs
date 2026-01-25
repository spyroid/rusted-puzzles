use fun_time::fun_time;

#[derive(Debug)]
enum Node {
    Terminal(char),
    Choice((usize, usize), (usize, usize)),
    Sequence(Vec<usize>),
}

#[fun_time(give_back)]
pub fn monster_messages(input: Vec<Box<str>>) -> (usize, usize) {
    let rules: Vec<_> = input
        .iter()
        .map(|s| {
            let parts = s.split(": ").collect::<Vec<_>>();
            let id = parts[0].parse::<usize>().unwrap();
            if parts[1].starts_with('"') {
                (id, Node::Terminal(parts[1].chars().nth(1).unwrap()))
            } else if parts[1].contains('|') {
                let ch: Vec<_> = parts[1]
                    .split('|')
                    .map(|s| {
                        s.split_whitespace()
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect();
                (id, Node::Choice((ch[0][0], ch[0][1]), (ch[1][0], ch[1][1])))
            } else {
                let seq = parts[1]
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                (id, Node::Sequence(seq))
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
