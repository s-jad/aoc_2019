use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut nums = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect_vec();

    nums[1] = 12;
    nums[2] = 2;

    let mut idx = 0;
    let mut finished = false;

    while !finished {
        let op = &nums.clone()[idx..idx + 4];
        match op[0] {
            1 => {
                nums[op[3]] = nums[op[1]] + nums[op[2]];
            }
            2 => {
                nums[op[3]] = nums[op[1]] * nums[op[2]];
            }
            99 => {
                finished = true;
            }
            _ => unreachable!(),
        }
        idx += 4;
    }

    nums[0]
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
