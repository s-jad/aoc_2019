use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn gcd(num1: usize, num2: usize) -> usize {
    let mut x = num1.max(num2);
    let mut y = num1.min(num2);

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    x
}

fn process(input: &str) -> usize {
    let mut moons = input
        .lines()
        .map(|l| {
            l.split_terminator(&['<', '>', ',', '='][..])
                .filter_map(|c| c.parse::<i32>().ok())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();

    let mut velocitys = vec![];

    for _ in 0..4 {
        velocitys.push((0, 0, 0));
    }

    let mut seen_x = HashSet::new();
    let mut x_loop = 0;
    let mut x_found = false;

    let mut seen_y = HashSet::new();
    let mut y_loop = 0;
    let mut y_found = false;

    let mut seen_z = HashSet::new();
    let mut z_loop = 0;
    let mut z_found = false;

    let mut found = false;

    let mut idx = 0;

    while !found {
        for i in 0..moons.len() {
            moons[i].0 += velocitys[i].0;
            moons[i].1 += velocitys[i].1;
            moons[i].2 += velocitys[i].2;
        }

        let current_v = velocitys.clone();

        for i in 0..moons.len() {
            for j in i..moons.len() {
                match (moons[i].0, moons[j].0) {
                    _ if moons[i].0 == moons[j].0 => {}
                    _ if moons[i].0 < moons[j].0 => {
                        velocitys[i].0 += 1;
                        velocitys[j].0 -= 1;
                    }
                    _ if moons[i].0 > moons[j].0 => {
                        velocitys[i].0 -= 1;
                        velocitys[j].0 += 1;
                    }
                    _ => unreachable!(),
                }
                match (moons[i].1, moons[j].1) {
                    _ if moons[i].1 == moons[j].1 => {}
                    _ if moons[i].1 < moons[j].1 => {
                        velocitys[i].1 += 1;
                        velocitys[j].1 -= 1;
                    }
                    _ if moons[i].1 > moons[j].1 => {
                        velocitys[i].1 -= 1;
                        velocitys[j].1 += 1;
                    }
                    _ => unreachable!(),
                }
                match (moons[i].2, moons[j].2) {
                    _ if moons[i].2 == moons[j].2 => {}
                    _ if moons[i].2 < moons[j].2 => {
                        velocitys[i].2 += 1;
                        velocitys[j].2 -= 1;
                    }
                    _ if moons[i].2 > moons[j].2 => {
                        velocitys[i].2 -= 1;
                        velocitys[j].2 += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut current_x = [0; 8];
        let mut current_y = [0; 8];
        let mut current_z = [0; 8];

        for i in 0..moons.len() {
            current_x[i] = moons[i].0;
            current_x[i + 4] = current_v[i].0;
            current_y[i] = moons[i].1;
            current_y[i + 4] = current_v[i].1;
            current_z[i] = moons[i].2;
            current_z[i + 4] = current_v[i].2;
        }

        if !seen_x.insert(current_x) && !x_found {
            x_found = true;
            x_loop = idx;
        }

        if !seen_y.insert(current_y) && !y_found {
            y_found = true;
            y_loop = idx;
        }

        if !seen_z.insert(current_z) && !z_found {
            z_found = true;
            z_loop = idx;
        }

        if x_found && y_found && z_found {
            found = true;
        }

        idx += 1;
    }

    lcm(lcm(x_loop, y_loop), z_loop)
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
