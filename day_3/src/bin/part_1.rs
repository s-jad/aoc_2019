use itertools::Itertools;
use std::{ops::Range, time::Instant};

fn ranges_intersect_point(
    s1: &(i32, i32),
    e1: &(i32, i32),
    s2: &(i32, i32),
    e2: &(i32, i32),
) -> Option<(i32, i32)> {
    if s1.0 == e1.0 && s2.1 == e2.1 {
        // s1-e1 is vertical and s2-e2 is horizontal
        if (s2.0.min(e2.0)..=s2.0.max(e2.0)).contains(&s1.0)
            && (s1.1.min(e1.1)..=s1.1.max(e1.1)).contains(&s2.1)
        {
            return Some((s1.0, s2.1));
        }
    } else if s1.1 == e1.1 && s2.0 == e2.0 {
        // s1-e1 is horizontal and s2-e2 is vertical
        if (s1.0.min(e1.0)..=s1.0.max(e1.0)).contains(&s2.0)
            && (s2.1.min(e2.1)..=s2.1.max(e2.1)).contains(&s1.1)
        {
            return Some((s2.0, s1.1));
        }
    }
    None
}

fn find_intersections(r1: &[Range<(i32, i32)>], r2: &[Range<(i32, i32)>]) -> Vec<(i32, i32)> {
    let mut intersections = vec![];

    for range_1 in r1.iter() {
        for range_2 in r2.iter() {
            if let Some(intersect) =
                ranges_intersect_point(&range_1.start, &range_1.end, &range_2.start, &range_2.end)
            {
                intersections.push(intersect);
            }
        }
    }

    intersections
}

fn calculate_ranges(wire: &[(char, i32)]) -> Vec<Range<(i32, i32)>> {
    let mut ranges = vec![];
    let mut w_start = (0, 0);
    let mut s = w_start;

    for op in wire {
        match op.0 {
            'L' => {
                w_start = (w_start.0 - op.1, w_start.1);
                let e = w_start;
                ranges.push(s..e);
            }
            'R' => {
                w_start = (w_start.0 + op.1, w_start.1);
                let e = w_start;
                ranges.push(s..e);
            }
            'U' => {
                w_start = (w_start.0, w_start.1 + op.1);
                let e = w_start;
                ranges.push(s..e);
            }
            'D' => {
                w_start = (w_start.0, w_start.1 - op.1);
                let e = w_start;
                ranges.push(s..e);
            }
            _ => unreachable!(),
        }
        let last_range = ranges.iter().last().unwrap();
        s = last_range.end;
    }

    ranges
}

fn process(input: &str) -> i32 {
    let (wire_1, wire_2) = input
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .into_iter()
                .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse::<i32>().unwrap()))
                .collect_vec()
        })
        .collect_tuple::<(_, _)>()
        .unwrap();

    let r1 = calculate_ranges(&wire_1);
    let r2 = calculate_ranges(&wire_2);

    let start = (0, 0);
    let intersections = find_intersections(&r1, &r2);

    let mut shortest = 1000000000i32;

    for intersection in intersections.into_iter().filter(|i| i != &(0, 0)) {
        let manhattan = (start.0 - intersection.0).abs() + (start.1 - intersection.1).abs();

        if manhattan < shortest {
            shortest = manhattan;
        }
    }

    shortest
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
