use fun_time::fun_time;

#[fun_time(give_back)]
fn combo_breaker(input: Vec<Box<str>>) -> usize {
    let keys = input
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    fn transform(subject: usize, value: usize) -> usize {
        (value * subject) % 20201227
    }

    fn transform_loop(subject: usize, loop_size: usize) -> usize {
        let mut value = 1;
        for _ in 0..loop_size {
            value = transform(subject, value);
        }
        value
    }

    fn find_loop_size(key: usize) -> usize {
        let mut value = 1;
        let mut loop_count = 0;
        while value != key {
            value = transform(7, value);
            loop_count += 1;
        }
        loop_count
    }

    transform_loop(keys[1], find_loop_size(keys[0]))
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::day25::combo_breaker;
    use crate::gears::{input_data_lines, print_result};

    #[test]
    fn aoc() {
        print_result(combo_breaker(input_data_lines()));
    }
}
