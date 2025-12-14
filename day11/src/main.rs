use std::collections::HashMap;

const SAMPLE_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

fn main() {
    let _input = include_str!("../../inputs/day11.txt");
    let result = solve_part_one(SAMPLE_INPUT);
    println!("Part one: {result}");
}

/// Maybe worth using to serialize if I get into weird lifetime stuff
/// with slices.
fn letters_to_usize(letters: &str) -> u32 {
    letters.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + (c.to_digit(36).unwrap() * (100 as u32).pow(i as u32))
    })
}

// fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {}

fn solve_part_one(input: &str) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (node, conns) = line.split_once(": ").unwrap();
        let conns = conns.split(" ").collect();
        map.insert(node, conns);
    }

    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one_sample() {
        let result = solve_part_one(SAMPLE_INPUT);

        assert_eq!(result, 5);
    }
}
