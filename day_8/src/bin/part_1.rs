use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let layer_size = 25 * 6;

    let layers = input
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|c| c.into_iter().filter_map(|c| c.to_digit(10)).collect_vec())
        .filter(|v| !v.is_empty())
        .collect_vec();

    let mut fewest_0 = 100000000;
    let mut fewest_0_layer = vec![];

    for layer in layers {
        let count_0 = layer.iter().filter(|&&n| n == 0u32).count();

        if count_0 < fewest_0 {
            fewest_0 = count_0;
            fewest_0_layer = layer;
        }
    }

    let count_1 = fewest_0_layer.iter().filter(|&&n| n == 1u32).count();
    let count_2 = fewest_0_layer.iter().filter(|&&n| n == 2u32).count();

    count_1 * count_2
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
