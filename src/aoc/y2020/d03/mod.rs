use fun_time::fun_time;

#[fun_time(give_back)]
pub fn toboggan_trajectory(input: Vec<Box<str>>) -> (usize, usize) {
    let width = input[0].len();
    let part1 = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(row, str)| str.chars().nth(((row - 1) * 3 + 3) % width).unwrap() == '#')
        .count();

    (part1, 0)
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
