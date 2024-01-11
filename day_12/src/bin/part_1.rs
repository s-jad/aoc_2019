use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> i32 {
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

    let steps = 1000;

    for x in 0..=steps {
        for i in 0..moons.len() {
            moons[i].0 += velocitys[i].0;
            moons[i].1 += velocitys[i].1;
            moons[i].2 += velocitys[i].2;
        }

        if x < steps {
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
        }
    }

    moons
        .iter()
        .zip(velocitys.iter())
        .map(|(m, v)| (m.0.abs() + m.1.abs() + m.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
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
