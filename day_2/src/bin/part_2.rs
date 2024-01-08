use itertools::Itertools;
use std::time::Instant;

fn test_combo(nums: Vec<usize>, verb: usize, noun: usize) -> (usize, usize, usize) {
    let mut inner_nums = nums.clone();
    inner_nums[1] = noun;
    inner_nums[2] = verb;
    let mut idx = 0;
    let mut finished = false;

    while !finished {
        let op = &inner_nums.clone()[idx..idx + 4];
        match op[0] {
            1 => {
                inner_nums[op[3]] = inner_nums[op[1]] + inner_nums[op[2]];
            }
            2 => {
                inner_nums[op[3]] = inner_nums[op[1]] * inner_nums[op[2]];
            }
            99 => {
                finished = true;
            }
            _ => unreachable!(),
        }
        idx += 4;
    }

    (inner_nums[0], noun, verb)
}

fn process(input: &str) -> usize {
    let nums = input
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect_vec();

    let program_inputs = (0..100).cartesian_product(0..100).collect_vec();

    let goal = 19690720;
    let mut idx = 0;
    let mut n = 0;
    let mut noun = 0;
    let mut verb = 0;

    while n != goal {
        (n, noun, verb) = test_combo(nums.clone(), program_inputs[idx].0, program_inputs[idx].1);
        idx += 1;
    }

    100 * noun + verb
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
