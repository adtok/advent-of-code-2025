use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input);
    println!("Part two: {result}");
}

fn solve_part_one(input: &str) -> usize {
    let (fresh_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();
    let fresh_ranges: Vec<(usize, usize)> = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    ingredient_ids
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|id| {
            fresh_ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(id))
        })
        .count()
}

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn overlap(a: (usize, usize), b: (usize, usize)) -> Option<(usize, usize)> {
    match (a, b) {
        (a, b) if (a.0..=a.1).contains(&b.0) => Some((a.0, max(a.1, b.1))),
        (a, b) if (b.0..=b.1).contains(&a.0) => Some((b.0, max(a.1, b.1))),
        _ => None,
    }
}

fn solve_part_two(input: &str) -> usize {
    let mut ranges: Vec<(usize, usize)> = vec![];
    for line in input.split_once("\n\n").unwrap().0.lines() {
        let (start, end) = line.split_once("-").unwrap();
        let mut range: (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
        loop {
            if let Some(idx) = ranges
                .iter()
                .position(|&other| overlap(range, other).is_some())
            {
                let overlapped = ranges[idx];
                range = overlap(range, overlapped).unwrap();
                ranges.remove(idx);
            } else {
                ranges.push(range);
                break;
            }
        }
    }
    ranges.iter().map(|range| (range.0..=range.1).count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_solve_part_one_sample() {
        let result = solve_part_one(SAMPLE_INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part_two_sample() {
        let result = solve_part_two(SAMPLE_INPUT);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_overlap_a_below_b() {
        let a = (3, 5);
        let b = (10, 14);

        assert_eq!(overlap(a, b), None);
    }

    #[test]
    fn test_overlap_b_below_a() {
        let a = (10, 14);
        let b = (3, 5);

        assert_eq!(overlap(a, b), None);
    }

    #[test]
    fn test_overlap_a_extends_b() {
        let a = (10, 14);
        let b = (12, 18);

        assert_eq!(overlap(a, b), Some((10, 18)));
    }

    #[test]
    fn test_overlap_b_extends_a() {
        let b = (10, 14);
        let a = (12, 18);

        assert_eq!(overlap(a, b), Some((10, 18)));
    }

    #[test]
    fn test_overlap_a_contains_b() {
        let a = (2, 6);
        let b = (3, 5);

        assert_eq!(overlap(a, b), Some((2, 6)));
    }

    #[test]
    fn test_overlap_b_contains_a() {
        let a = (3, 5);
        let b = (2, 6);

        assert_eq!(overlap(a, b), Some((2, 6)));
    }
}
