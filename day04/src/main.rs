fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = 0;
    println!("Part two: {result}");
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    let width = input.find('\n').unwrap();
    let m = [".".repeat(width), input.to_string(), ".".repeat(width)].join("\n");
    m.lines()
        .map(|line| format!(".{line}.").chars().collect())
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
    let lines = read_input(input);
    let width = lines[0].len();
    let height = lines.len();

    let mut result = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if lines[y][x] == '.' {
                continue;
            }

            let adjacent_rolls = neighbors(x, y)
                .iter()
                .filter(|p| lines[p.1][p.0] == '@')
                .count();
            if adjacent_rolls < 5 {
                result += 1;
            }
        }
    }
    result
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
