fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = 0;
    println!("Part two: {result}");
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let width = input.find('\n').unwrap();
    let padded: Vec<Vec<char>> = [".".repeat(width), input.to_string(), ".".repeat(width)]
        .join("\n")
        .lines()
        .map(|line| format!(".{line}.").chars().collect())
        .collect();
    padded
        .iter()
        .enumerate()
        .map(|row| {
            row.1
                .iter()
                .enumerate()
                .map(|col| {
                    let x = col.0;
                    let y = row.0;
                    if padded[y][x] == '.' {
                        0
                    } else {
                        neighbors(x, y)
                            .iter()
                            .filter(|p| padded[p.1][p.0] == '@')
                            .count()
                    }
                })
                .collect()
        })
        .collect()
}

fn neighbors(x: usize, y: usize) -> [(usize, usize); 9] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn solve_part_one(input: &str) -> usize {
    let layout = parse_input(input);

    let mut result = 0;

    let result = layout
        .iter()
        .map(|row| row.iter().filter(|&count| (1..5).contains(count)).count())
        .sum();

    result
}

fn remove_rolls(layout: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize) {
    return (layout.clone(), 0);
}

fn solve_part_two(input: &str) -> usize {
    let lines = parse_input(input);
    let width = lines[0].len();
    let height = lines.len();

    43
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one_sample() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = solve_part_one(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_solve_part_two_sample() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = 43;

        assert_eq!(result, 43);
    }
}
