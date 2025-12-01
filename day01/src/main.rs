fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input, 50);
    println!("Part two: {result}");
}

fn solve_part_one(input: &str) -> usize {
    let mut position: isize = 50;
    let mut count = 0;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value = value.parse::<isize>().unwrap();
        let value = if dir == "R" { value } else { -value };
        position += value;
        position = position % 100;
        if position == 0 {
            count += 1;
        }
    }
    count
}

fn solve_part_two(input: &str, pos: isize) -> usize {
    let mut position: isize = pos;
    let mut count = 0;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value = value.parse::<isize>().unwrap();
        let quotient: usize = (value / 100).try_into().unwrap();
        count += quotient;
        let change = value % 100;
        let change = if dir == "R" { change } else { -change };

        if position != 0 && (position + change < 0 || position + change > 100) {
            count += 1;
        }

        position += change;

        position = position % 100;
        if position < 0 {
            position = 100 + position;
        }
        if position == 0 && change != 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part_one(input), 3);
    }

    #[test]
    fn test_part2_example() {
        let position = 50;
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part_two(input, position), 6);
    }

    #[test]
    fn test_part2_example_subset_one() {
        let position = 50;
        let input = "L68
L30
R48
L5
R60";
        assert_eq!(solve_part_two(input, position), 3);
    }

    #[test]
    fn test_quotients_case_one() {
        let position = 1;
        let input = "L1
L1000";

        assert_eq!(solve_part_two(input, position), 11);
    }

    #[test]
    fn test_quotients_case_two() {
        let position = 1;
        let input = "R199";
        assert_eq!(solve_part_two(input, position), 2);
    }

    #[test]
    fn test_quotients_case_three() {
        let position = 0;
        let input = "L1000";

        assert_eq!(solve_part_two(input, position), 10);
    }
}
