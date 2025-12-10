use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../inputs/day10.txt");
    let result = solve_part_one(input);
    println!("Part one: {result}");
}

struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    _joltages: Vec<usize>,
}

impl Machine {
    fn parse_lights(lights: &str) -> u16 {
        lights
            .trim_matches(&['[', ']'])
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .enumerate()
            .fold(0, |acc, (i, b)| acc | (b << i))
    }

    fn parse_buttons(buttons: &str) -> Vec<u16> {
        buttons
            .split(" ")
            .map(|raw| {
                raw.trim_matches(&['(', ')'])
                    .split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .fold(0, |acc, v| acc | (1 << v))
            })
            .collect()
    }

    fn parse_joltages(joltages: &str) -> Vec<usize> {
        joltages
            .trim_matches(&['{', '}'])
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect()
    }

    fn from_line(line: &str) -> Self {
        let (lights, rest) = line.split_once(" ").unwrap();
        let (buttons, joltages) = rest.rsplit_once(" ").unwrap();
        let lights = Self::parse_lights(lights);
        let buttons = Self::parse_buttons(buttons);
        let joltages = Self::parse_joltages(joltages);
        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

fn solve_machine(machine: &Machine) -> u16 {
    let mut queue: VecDeque<(u16, u16)> = VecDeque::from([(0, 0)]);
    while let Some((lights, iters)) = queue.pop_front() {
        for button in &machine.buttons {
            let new_ls = lights ^ button;
            if new_ls == machine.lights {
                return iters + 1;
            }
            queue.push_back((new_ls, iters + 1));
        }
    }
    panic!("Could not find the pattern!");
}

fn solve_part_one(input: &str) -> u16 {
    let machines: Vec<Machine> = input.lines().map(Machine::from_line).collect();
    machines.iter().map(solve_machine).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_solve_part_one_sample() {
        let result = solve_part_one(SAMPLE_INPUT);
        assert_eq!(result, 7);
    }
}
