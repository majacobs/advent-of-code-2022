use crate::harness::Harness;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<u8>;
    type Part1Output = i32;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input.into_bytes()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        input
            .iter()
            .map(|b| match b {
                b'(' => 1,
                b')' => -1,
                _ => unreachable!(),
            })
            .sum()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let mut floor = 0;
        for (i, b) in input.iter().enumerate() {
            match b {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => unreachable!(),
            }
            if floor < 0 {
                return i + 1;
            }
        }

        unreachable!()
    }
}
