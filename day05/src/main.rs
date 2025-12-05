fn main() {
    let input = include_str!("../../inputs/day05.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one_sample() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = solve_part_one(input);
        assert_eq!(result, 3);
    }
}
