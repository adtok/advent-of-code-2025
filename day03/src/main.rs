fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
}

fn solve_part_one(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let nums: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let len = nums.len();
        let mut tens = u32::MIN;
        let mut idx = 0;
        for i in 0..len - 1 {
            if nums[i] > tens {
                tens = nums[i];
                idx = i;
            }
        }
        let mut ones = u32::MIN;
        for i in idx + 1..len {
            if nums[i] > ones {
                ones = nums[i];
            }
        }
        let max = 10 * tens + ones;
        result += max;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one_sample() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = solve_part_one(input);

        assert_eq!(result, 357);
    }
}
