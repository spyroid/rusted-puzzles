use crate::gears::point::Point;
use fun_time::fun_time;

#[fun_time(give_back)]
pub fn toboggan_trajectory(input: Vec<Box<str>>) -> (usize, usize) {
    let width = input[0].len();

    let count_trees = |p: &Point| -> usize {
        input
            .iter()
            .step_by(p.y)
            .enumerate()
            .filter(|(i, row)| row.as_bytes()[i * p.x % width] == b'#')
            .count()
    };

    let slopes: Vec<_> = vec![
        Point { x: 3, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ]
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
