use crate::harness::Harness;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<char>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input.chars().collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        const SIZE: usize = 4;
        let window_start = input
            .windows(SIZE)
            .enumerate()
            .find(|(_, chars)| all_unique(chars))
            .unwrap()
            .0;
        window_start + SIZE
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        const SIZE: usize = 14;
        let window_start = input
            .windows(SIZE)
            .enumerate()
            .find(|(_, chars)| all_unique(chars))
            .unwrap()
            .0;
        window_start + SIZE
    }
}

fn all_unique(chars: &[char]) -> bool {
    let size = chars.len();
    for i in 0..size {
        for j in (i + 1)..size {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}
