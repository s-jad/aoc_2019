use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let layer_size = 25 * 6;

    let max_0 = input
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|c| {
            c.into_iter().filter_map(|c| c.to_digit(10)).fold(
                (0, 0, 0),
                |(c0, c1, c2), n| match n {
                    0 => (c0 + 1, c1, c2),
                    1 => (c0, c1 + 1, c2),
                    2 => (c0, c1, c2 + 1),
                    _ => unreachable!(),
                },
            )
        })
        .filter(|c| c != &(0, 0, 0))
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap();

    max_0.1 * max_0.2
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
