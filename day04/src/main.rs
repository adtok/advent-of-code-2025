type FactoryFloor = Vec<Vec<usize>>;

fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input);
    println!("Part two: {result}");
}

fn parse_input(input: &str) -> FactoryFloor {
    let width = input.find('\n').unwrap();
    let padded: Vec<Vec<char>> = [
        ".".repeat(width),
        input.trim().to_string(),
        ".".repeat(width),
    ]
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
                        let ns = neighbors(x, y);
                        let result = ns.iter().filter(|p| padded[p.1][p.0] == '@').count();
                        result
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
    layout
        .iter()
        .map(|row| row.iter().filter(|&ns| (1..5).contains(ns)).count())
        .sum()
}

fn remove_rolls(layout: &FactoryFloor) -> (FactoryFloor, usize) {
    let mut new_layout = layout.clone();
    let height = new_layout.len();
    let width = new_layout[0].len();
    let mut removed = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if new_layout[y][x] < 5 && new_layout[y][x] != 0 {
                new_layout[y][x] = 0;
                removed += 1;
            }
        }
    }
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if new_layout[y][x] != 0 {
                new_layout[y][x] = neighbors(x, y)
                    .iter()
                    .filter(|p| new_layout[p.1][p.0] != 0)
                    .count();
            }
        }
    }

    return (new_layout, removed);
}

fn solve_part_two(input: &str) -> usize {
    let mut layout = parse_input(input);

    let mut result = 0;
    loop {
        let (new_layout, n_removed) = remove_rolls(&layout);
        result += n_removed;
        if n_removed == 0 {
            break;
        }
        layout = new_layout;
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
        let result = solve_part_two(input);

        assert_eq!(result, 43);
    }
}
