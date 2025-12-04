fn main() {
    let input = include_str!("../../inputs/day04.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input);
    println!("Part two: {result}");
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
                result += 1;
            }
        }
    }
    result
}

fn remove_rolls(counts: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let width = counts[0].len();
    let height = counts[1].len();
    let pre = counts
        .iter()
        .enumerate()
        .map(|row| {
            row.1
                .iter()
                .enumerate()
                .map(|col| {
                    let x = col.0;
                    let y = row.0;
                    if counts[y][x] < 5 { 0 } else { counts[y][x] }
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();
    let get_ns = |x, y| {
        neighbors(x as isize, y as isize)
            .iter()
            .filter(|n| n.0 > -1 && n.0 < width as isize && n.1 > -1 && n.1 < height as isize)
            .map(|n| pre[n.1 as usize][n.0 as usize].clone())
            .filter(|&n| n > 0)
            .count()
    };
    pre.iter()
        .enumerate()
        .map(|row| {
            row.1
                .iter()
                .enumerate()
                .map(|col| {
                    let x = col.0;
                    let y = row.0;
                    if pre[y][x] == 0 { 0 } else { get_ns(x, y) }
                })
                .collect()
        })
        .collect()
}

fn count_empty(counts: &Vec<Vec<usize>>) -> usize {
    counts
        .iter()
        .map(|x| x.iter().filter(|&y| *y == 0).count())
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let get_ns = |x, y| {
        neighbors(x as isize, y as isize)
            .iter()
            .filter(|n| n.0 > -1 && n.0 < width as isize && n.1 > -1 && n.1 < height as isize)
            .map(|n| lines[n.1 as usize][n.0 as usize].clone())
            .collect::<Vec<char>>()
    };

    let initial_counts: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|line| {
            line.1
                .chars()
                .enumerate()
                .map(|t_cs| {
                    let x = t_cs.0;
                    let y = line.0;
                    t_cs.1;
                    if lines[y][x] == '.' {
                        return 0;
                    }
                    let ns = get_ns(x, y);
                    let adjacent_rolls = ns
                        .iter()
                        .filter(|&&space| space == '@')
                        .collect::<Vec<&char>>()
                        .len();
                    adjacent_rolls
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut counts = remove_rolls(&initial_counts.clone());
    loop {
        let prev = count_empty(&counts);
        counts = remove_rolls(&counts);
        let curr = count_empty(&counts);
        if prev == curr {
            break;
        }
    }

    count_empty(&counts) - count_empty(&initial_counts)
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
