use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn count_orbits(orbits: &HashMap<&str, &str>, obj: &str) -> usize {
    let mut count = 0;
    let mut current = obj;

    while current != "COM" {
        count += 1;
        current = orbits.get(current).unwrap();
    }

    count
}

fn total_orbits(orbits: &HashMap<&str, &str>) -> usize {
    let mut total = 0;

    for (&b, _) in orbits.iter() {
        total += count_orbits(orbits, b);
    }

    total
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

    total_orbits(&orbits)
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
