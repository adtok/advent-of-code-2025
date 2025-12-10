fn main() {
    let input = include_str!("../../inputs/day09.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
}

type Point = (usize, usize);

fn area(p: Point, q: Point) -> usize {
    let width = p.0.abs_diff(q.0) + 1;
    let height = p.1.abs_diff(q.1) + 1;
    width * height
}

fn solve_part_one(input: &str) -> usize {
    let points: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let mut max_area = 0;
    for i in 0..(points.len() - 1) {
        for j in 1..points.len() {
            let a = area(points[i], points[j]);
            if a > max_area {
                max_area = a;
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_solve_part_one_sample() {
        let result = solve_part_one(SAMPLE_INPUT);
        assert_eq!(result, 50);
    }
}
