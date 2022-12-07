use crate::harness::Harness;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<(Range, Range)>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input
            .lines()
            .map(|s| {
                let mut ranges = s.split([',', '-']);
                (
                    Range {
                        start: ranges.next().unwrap().parse().unwrap(),
                        end: ranges.next().unwrap().parse().unwrap(),
                    },
                    Range {
                        start: ranges.next().unwrap().parse().unwrap(),
                        end: ranges.next().unwrap().parse().unwrap(),
                    },
                )
            })
            .collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        input
            .iter()
            .filter(|(a, b)| {
                a.start >= b.start && a.end <= b.end || b.start >= a.start && b.end <= a.end
            })
            .count()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        input
            .iter()
            .filter(|(a, b)| a.start <= b.end && a.end >= b.start)
            .count()
    }
}

pub struct Range {
    start: u32,
    end: u32,
}
