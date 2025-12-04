fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
}

fn neighbors(x: isize, y: isize) -> [(isize, isize); 9] {
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
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut result = 0;

    for y in 0..height {
        for x in 0..width {
            let ns: Vec<char> = neighbors(x as isize, y as isize)
                .iter()
                .filter(|n| n.0 > -1 && n.0 < width as isize && n.1 > -1 && n.1 < height as isize)
                .map(|n| lines[n.1 as usize][n.0 as usize].clone())
                .collect();
            if lines[y][x] == '.' {
                continue;
            }
            let adjacent_rolls = ns
                .iter()
                .filter(|&&space| space == '@')
                .collect::<Vec<&char>>()
                .len();
            if adjacent_rolls < 5 {
                println!("{x}, {y}");
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
}
