use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn find_fork(orbits: &HashMap<&str, &str>) -> usize {
    let mut start = "YOU";
    let mut end = "SAN";
    let mut start_path = vec![];
    let mut end_path = vec![];

    while start != "COM" {
        start = orbits.get(start).unwrap();
        start_path.push(start);
    }

    while !start_path.contains(&end) {
        end = orbits.get(end).unwrap();
        end_path.push(end);
    }

    let sl = start_path
        .iter()
        .position(|c| c == end_path.last().unwrap())
        .unwrap();

    sl + end_path.len() - 1
}

fn process(input: &str) -> usize {
    let lines = input.lines();
    let mut orbits = HashMap::new();

    for l in lines {
        let mut parts = l.split(')');
        let p0 = parts.next().unwrap();
        let p1 = parts.next().unwrap();
        orbits.insert(p1, p0);
    }

    find_fork(&orbits)
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
