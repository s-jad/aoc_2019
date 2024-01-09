use itertools::Itertools;
use std::time::Instant;

fn valid_password(num: String) -> bool {
    let mut no_decrease = true;

    for (c1, c2) in num.chars().tuple_windows() {
        if c1 > c2 {
            no_decrease = false;
        }
    }

    let num_counts = num.chars().counts().iter().any(|(_, i)| i == &2);

    num_counts && no_decrease
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
        let num = i.to_string();

        if valid_password(num) {
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
