use crate::harness::Harness;
use std::cmp::Reverse;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Option<i32>>;
    type Part1Output = i32;
    type Part2Output = i32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input.lines().map(|l| l.parse().ok()).collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut max = 0;
        let mut running = 0;
        for o in input {
            if let Some(i) = o {
                running += i;
            } else {
                if running > max {
                    max = running;
                }
                running = 0;
            }
        }
        max
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut sums = vec![];
        let mut running = 0;
        for o in input {
            if let Some(i) = o {
                running += i;
            } else {
                sums.push(running);
                running = 0;
            }
        }
        sums.sort_unstable_by_key(|i| Reverse(*i));
        sums.into_iter().take(3).sum()
    }
}
