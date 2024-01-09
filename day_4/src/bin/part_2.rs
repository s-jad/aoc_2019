use itertools::Itertools;
use std::time::Instant;

fn valid_password(num: usize) -> bool {
    let mut counts = [0; 10];
    let mut prev = 0;
    let mut decreasing = false;

    for d in num.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
        counts[d as usize] += 1;
        if d < prev {
            decreasing = true;
        }
        prev = d;
    }

    !decreasing && counts.iter().any(|&count| count == 2)
}

fn process(input: &str) -> usize {
    let (s, e) = input
        .trim()
        .split('-')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut valid_count = 0;
    for i in s..e {
        if valid_password(i) {
            valid_count += 1;
        }
    }

    valid_count
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
