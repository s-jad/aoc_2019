use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let layer_size = 25 * 6;

    let min_layer = input
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|c| c.into_iter().filter_map(|c| c.to_digit(10)).collect_vec())
        .filter(|v| !v.is_empty())
        .fold((100000, (0, 0)), |(min, (count_1, count_2)), layer| {
            let mut map = HashMap::new();

            for &c in &layer {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }

            let c_0 = map.get(&0).unwrap_or(&0);
            let c_1 = map.get(&1).unwrap_or(&0);
            let c_2 = map.get(&2).unwrap_or(&0);

            if c_0 < &min {
                (*c_0, (*c_1, *c_2))
            } else {
                (min, (count_1, count_2))
            }
        });

    min_layer.1 .0 * min_layer.1 .1
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
