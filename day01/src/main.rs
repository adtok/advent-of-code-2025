fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let result = solve(input);
    println!("Part one: {result}");
}

fn solve(input: &str) -> usize {
    let mut position: isize = 50;
    let mut count = 0;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value = value.parse::<isize>().unwrap();
        if dir == "R" {
            position += value;
        } else {
            position -= value;
        }
        position = position % 100;
        if position == 0 {
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
        assert_eq!(solve(input), 3);
    }
}
