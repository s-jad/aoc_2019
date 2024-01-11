use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let layer_row = 25;
    let layer_depth = 6;

    let images = input
        .chars()
        .chunks(layer_row * layer_depth)
        .into_iter()
        .map(|c| c.into_iter().filter_map(|c| c.to_digit(10)).collect_vec())
        .collect_vec();

    let mut final_image: Vec<char> = vec![];
    let i_len = images.len();
    for i in 0..images[0].len() {
        let mut idx = 0;
        while idx < i_len {
            match images[idx][i] {
                0 => {
                    final_image.push('.');
                    idx += i_len;
                }
                1 => {
                    final_image.push('#');
                    idx += i_len;
                }
                2 => {
                    idx += 1;
                }
                _ => unreachable!(),
            }
        }
    }

    for i in 0..final_image.len() {
        if i % layer_row == 0 {
            println!("");
        }
        print!("{:?}", final_image[i]);
    }

    1
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();
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
