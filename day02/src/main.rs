fn main() {
    let input = include_str!("../../inputs/day02.txt").trim();
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input);
    println!("Part two: {result}");
}

fn is_invalid(id: usize) -> bool {
    if id < 11 {
        return false;
    }
    let value = id.to_string();
    let len = value.len();
    if len % 2 != 0 {
        return false;
    }
    let (first, second) = value.split_at(len / 2);
    first == second
}

fn solve_part_one(input: &str) -> usize {
    let mut result = 0;
    for range in input.split(",") {
        let (lower, upper) = range.split_once("-").unwrap();
        let lower = lower.parse::<usize>().unwrap();
        let upper = upper.parse::<usize>().unwrap();
        for id in lower..=upper {
            if is_invalid(id) {
                result += id;
            }
        }
    }
    result
}

fn repeats(value: &str) -> bool {
    let len = value.len();
    if len < 2 {
        return false;
    }
    for i in 2..=len {
        if len % i != 0 {
            continue;
        }
        let (check, _) = value.split_at(len / i);
        if check.repeat(i) == value {
            return true;
        }
    }
    return false;
}

fn solve_part_two(input: &str) -> usize {
    let mut result = 0;
    for range in input.split(",") {
        let (lower, upper) = range.split_once("-").unwrap();
        let lower = lower.parse::<usize>().unwrap();
        let upper = upper.parse::<usize>().unwrap();
        for id in lower..=upper {
            if repeats(id.to_string().as_str()) {
                result += id;
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";
        println!("hi {}", input);
        assert_eq!(solve_part_one(input), 1227775554);
    }

    #[test]
    fn test_solve_part_two_sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";
        println!("hi {}", input);
        assert_eq!(solve_part_two(input), 4174379265);
    }
}
