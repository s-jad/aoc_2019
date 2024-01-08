use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            let mut n = (l.parse::<isize>().unwrap() / 3) - 2;
            let mut total = 0;

            while n > 0 {
                total += n;
                n = (n / 3) - 2;
            }
            total
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
