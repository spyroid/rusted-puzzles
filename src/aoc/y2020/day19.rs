use fun_time::fun_time;

#[fun_time(give_back)]
pub fn monster_messages(input: Vec<Box<str>>) -> (usize, usize) {
    let rules: Vec<_> = input
        .iter()
        .map(|s| {
            let parts = s.split(": ").collect::<Vec<_>>();
            let parts_right: Vec<_> = parts[1]
                .replace("\"a\"", "-1")
                .replace("\"b\"", "-2")
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            (parts[0].parse::<i32>().unwrap(), parts_right)
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
