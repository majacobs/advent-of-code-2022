use crate::harness::Harness;
use md5::{Digest, Md5};

pub struct Solution;

impl Harness for Solution {
    type Parsed = String;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        for i in 1u32.. {
            let result = Md5::digest(format!("{}{}", input, i));
            if result[0] == 0 && result[1] == 0 && result[2] & 0b11110000 == 0 {
                return i;
            }
        }

        unreachable!()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        for i in 1u32.. {
            let result = Md5::digest(format!("{}{}", input, i));
            if result[0] == 0 && result[1] == 0 && result[2] == 0 {
                return i;
            }
        }

        unreachable!()
    }
}
