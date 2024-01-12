use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> usize {
    let mut mine = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = mine.len();
    let width = mine[0].len();

    let mut seen = HashMap::new();
    let cycle_end = 1000000000;
    let mut loop_found = false;
    let mut loop_start = 0;
    let mut loop_end = 0;
    let mut outer_cycle_idx = 0;
    let dirs = [(-1i32, 0i32), (0i32, -1i32), (1i32, 0i32), (0i32, 1i32)];

    while !loop_found {
        outer_cycle_idx += 1;
        let mut cycle_idx = 0;

        while cycle_idx < 4 {
            let mut num_moves = 1;
            let cd = dirs[cycle_idx];
            while num_moves > 0 {
                num_moves = 0;

                if cd.0 + cd.1 > 0 {
                    for y in 0..(height - cd.0.abs() as usize) {
                        for x in 0..(width - cd.1.abs() as usize) {
                            if mine[(y as i32 + cd.0) as usize][(x as i32 + cd.1) as usize] == '.'
                                && mine[y][x] == 'O'
                            {
                                mine[(y as i32 + cd.0) as usize][(x as i32 + cd.1) as usize] = 'O';
                                mine[y][x] = '.';
                                num_moves += 1;
                            }
                        }
                    }
                } else {
                    for y in (cd.0.abs() as usize)..height {
                        for x in (cd.1.abs() as usize)..width {
                            if mine[(y as i32 + cd.0) as usize][(x as i32 + cd.1) as usize] == '.'
                                && mine[y][x] == 'O'
                            {
                                mine[(y as i32 + cd.0) as usize][(x as i32 + cd.1) as usize] = 'O';
                                mine[y][x] = '.';
                                num_moves += 1;
                            }
                        }
                    }
                }
            }
            cycle_idx += 1;
        }

        let s = mine
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<String>();

        if seen.contains_key(&s) {
            loop_found = true;
            loop_start = *seen.get(&s).unwrap();
            loop_end = outer_cycle_idx;
        } else {
            seen.insert(s, outer_cycle_idx);
        }
    }

    let l_length = loop_end - loop_start;

    let effective_cycles = cycle_end - loop_start;
    let pos_in_loop = effective_cycles % l_length;
    let final_cycle_num = pos_in_loop + loop_start;

    let final_str = seen
        .iter()
        .find_map(|(k, v)| if v == &final_cycle_num { Some(k) } else { None })
        .unwrap();

    final_str
        .chars()
        .enumerate()
        .map(|(i, c)| if c == 'O' { height - (i / height) } else { 0 })
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
