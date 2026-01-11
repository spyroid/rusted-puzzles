use fun_time::fun_time;

#[fun_time(give_back)]
pub fn toboggan_trajectory(input: Vec<Box<str>>) -> (usize, usize) {
    let count_trees = |p: &(usize, usize)| -> usize {
        input
            .iter()
            .step_by(p.1)
            .enumerate()
            .filter(|(i, row)| row.as_bytes()[i * p.0 % input[0].len()] == b'#')
            .count()
    };

    let slopes: Vec<_> = vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|p| count_trees(p))
        .collect();

    let part1 = slopes[0];
    let part2 = slopes.iter().fold(1usize, |acc, p| acc * *p);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::d03::toboggan_trajectory;
    use crate::gears::{input_data_lines, print_debug};

    #[test]
    fn aoc() {
        print_debug(toboggan_trajectory(input_data_lines()));
    }
}
