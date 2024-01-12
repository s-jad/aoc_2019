use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut mine = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = mine.len();
    let width = mine[0].len();
    let mut num_moves = 1;

    while num_moves > 0 {
        num_moves = 0;

        for y in 1..height {
            for x in 0..width {
                if mine[y - 1][x] == '.' && mine[y][x] == 'O' {
                    mine[y - 1][x] = 'O';
                    mine[y][x] = '.';
                    num_moves += 1;
                }
            }
        }
    }

    mine.iter()
        .enumerate()
        .map(|(y, v)| -> usize { v.iter().filter(|&c| c == &'O').count() * (height - y) })
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
