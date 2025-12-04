fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
    let result = solve_part_two(input, 12);
    println!("Part two: {result}");
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

fn n_largest(nums: Vec<u32>, n_digits: usize) -> usize {
    let len = nums.len();
    let mut result: usize = 0;
    let mut idx = 0;
    for n in (0..n_digits).rev() {
        let mut max = u32::MIN;
        let start = if result == 0 { idx } else { idx + 1 };
        for i in start..len - (n) {
            if nums[i] > max {
                max = nums[i];
                idx = i;
            }
        }
        result = result * 10 + max as usize;
    }
    return result;
}

fn solve_part_two(input: &str, n_digits: usize) -> usize {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .map(|nums| n_largest(nums, n_digits))
        .sum()
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

    #[test]
    fn test_solve_part_one_n_largest() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = solve_part_two(input, 2);

        assert_eq!(result, 357);
    }

    #[test]
    fn test_solve_part_two_sample() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = solve_part_two(input, 12);

        assert_eq!(result, 3121910778619);
    }
}
