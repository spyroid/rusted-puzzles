use fun_time::fun_time;

#[fun_time(give_back)]
pub fn report_repair(input: Vec<u32>) -> (u64, u64) {
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if part1 == 0 && (input[i] + input[j]) == 2020 {
                    part1 = (input[i] * input[j]) as u64;
                }
                if part2 == 0 && (input[i] + input[j] + input[k]) == 2020 {
                    part2 = (input[i] * input[j] * input[k]) as u64;
                }
                if (part1 * part2) > 0 {
                    break;
                }
            }
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use crate::aoc::y2020::day1::report_repair;
    use crate::gears::{input_data_u32s, print_result};

    #[test]
    fn day1_test() {
        print_result(report_repair(input_data_u32s()));
    }
}
