fn main() {
    const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    let input = include_str!("../../inputs/day06.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");

    for line in SAMPLE_INPUT.lines() {
        println!("{:?}", line.split_whitespace().collect::<Vec<&str>>());
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_str(val: &str) -> (usize, Op) {
        match val {
            "+" => (0, Op::Add),
            "*" => (1, Op::Mul),
            _ => panic!("Unexpected '{val}'"),
        }
    }
}

fn solve_part_one(input: &str) -> usize {
    input
        .lines()
        .rfold(None, |acc, line| {
            let result = if let Some(accs) = acc {
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .iter()
                    .zip(accs)
                    .map(|(n, (m, op))| match op {
                        Op::Add => (n + m, op),
                        Op::Mul => (n * m, op),
                    })
                    .collect::<Vec<(usize, Op)>>()
            } else {
                line.split_whitespace()
                    .map(Op::from_str)
                    .collect::<Vec<(usize, Op)>>()
            };
            Some(result)
        })
        .unwrap()
        .iter()
        .map(|(n, _)| n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = ".123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_solve_part_one_sample() {
        let result = solve_part_one(SAMPLE_INPUT);

        assert_eq!(result, 4277556);
    }
}
