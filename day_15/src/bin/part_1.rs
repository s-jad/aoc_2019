use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    input
        .split_terminator(&[',', '\n'][..])
        .filter(|&s| s != "," && s != "\n")
        .map(|s| {
            let num = s.as_bytes();

            num.into_iter()
                .map(|n| *n as usize)
                .fold(0, |acc, n| ((acc + n) * 17) % 256) as usize
        })
        .sum()
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
