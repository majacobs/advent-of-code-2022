use crate::harness::Harness;
use std::str::FromStr;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Instruction>;
    type Part1Output = i32;
    type Part2Output = String;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut reg_x = 1;
        let mut cycle = 0;
        let mut next_measurement = 20;
        let mut sum = 0;
        for inst in input.iter() {
            let pre_x = reg_x;
            match inst {
                Instruction::Noop => {
                    cycle += 1;
                }
                Instruction::AddX(v) => {
                    reg_x += v;
                    cycle += 2;
                }
            }
            if cycle >= next_measurement {
                sum += next_measurement * pre_x;
                next_measurement += 40;
            }
        }

        sum
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let mut instructions = input.iter();
        let mut next = || instructions.next().map(|i| (*i, 1));
        let mut state = next().unwrap();
        let mut reg_x = 1;

        let mut rendered = String::new();

        for position in (0..40).cycle() {
            if position == 0 {
                rendered.push('\n');
            }
            if position == reg_x - 1 || position == reg_x || position == reg_x + 1 {
                rendered.push('#');
            } else {
                rendered.push('.');
            }

            match state {
                (Instruction::Noop, 1) => match next() {
                    Some(s) => state = s,
                    None => break,
                },
                (Instruction::AddX(v), 1) => {
                    state = (Instruction::AddX(v), 2);
                }
                (Instruction::AddX(v), 2) => {
                    reg_x += v;
                    match next() {
                        Some(s) => state = s,
                        None => break,
                    }
                }
                _ => unreachable!(),
            }
        }

        rendered
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        match parts.next().unwrap() {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let payload = parts.next().unwrap().parse().unwrap();
                Ok(Self::AddX(payload))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day10-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 13140);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day10-1").unwrap();
        let input = s.parse(raw);
        let expected = String::from(
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
        );
        assert_eq!(s.part2(&input), expected);
    }
}
