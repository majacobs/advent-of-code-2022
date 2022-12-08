use crate::harness::Harness;
use std::collections::HashSet;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Direction>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input.into_bytes().iter().map(|b| (*b).into()).collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut visited = HashSet::new();
        let mut location = (0, 0);
        visited.insert(location);

        for d in input.iter() {
            location = d.apply(location);
            visited.insert(location);
        }

        visited.len()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        let mut visited = HashSet::new();
        let mut santa = (0, 0);
        let mut robosanta = (0, 0);
        visited.insert(santa);

        for (i, d) in input.iter().enumerate() {
            if i % 2 == 0 {
                santa = d.apply(santa);
                visited.insert(santa);
            } else {
                robosanta = d.apply(robosanta);
                visited.insert(robosanta);
            }
        }

        visited.len()
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, location: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (location.0, location.1 + 1),
            Direction::Down => (location.0, location.1 - 1),
            Direction::Left => (location.0 - 1, location.1),
            Direction::Right => (location.0 + 1, location.1),
        }
    }
}

impl From<u8> for Direction {
    fn from(b: u8) -> Self {
        match b {
            b'^' => Self::Up,
            b'v' => Self::Down,
            b'<' => Self::Left,
            b'>' => Self::Right,
            _ => unreachable!(),
        }
    }
}
