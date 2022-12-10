use crate::harness::Harness;
use std::collections::HashSet;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Movement>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input
            .lines()
            .map(|l| {
                let (d, n) = l.split_once(' ').unwrap();
                let direction = match d {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => unreachable!(),
                };
                let number = n.parse().unwrap();
                Movement { direction, number }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut head = Position(0, 0);
        let mut tail = Position(0, 0);

        let mut visited = HashSet::new();
        visited.insert(tail);

        for movement in input.iter() {
            for _ in 0..movement.number {
                head.apply(movement.direction);
                tail.snap_to(&head);
                visited.insert(tail);
            }
        }

        visited.len()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        const LENGTH: usize = 10;
        let mut chain = [Position(0, 0); LENGTH];

        let mut visited = HashSet::new();
        visited.insert(chain[LENGTH - 1]);

        for movement in input.iter() {
            for _ in 0..movement.number {
                chain[0].apply(movement.direction);
                for i in 0..(LENGTH - 1) {
                    let leader = chain[i];
                    chain[i + 1].snap_to(&leader);
                }
                visited.insert(chain[LENGTH - 1]);
            }
        }

        visited.len()
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Position(i32, i32);

impl Position {
    fn apply(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.1 -= 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }

    fn snap_to(&mut self, other: &Self) {
        let d0 = other.0 - self.0;
        let d1 = other.1 - self.1;

        if d0.abs() > 1 {
            self.0 += d0.signum();
            if d1.abs() != 0 {
                self.1 += d1.signum();
            }
        } else if d1.abs() > 1 {
            self.1 += d1.signum();
            if d0.abs() != 0 {
                self.0 += d0.signum();
            }
        }
    }
}

pub struct Movement {
    direction: Direction,
    number: u32,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn part1_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day09-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part1(&input), 13);
    }

    #[test]
    fn part2_sample1() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day09-1").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 1);
    }

    #[test]
    fn part2_sample2() {
        let s = Solution {};
        let raw = read_to_string("samples/2022/day09-2").unwrap();
        let input = s.parse(raw);
        assert_eq!(s.part2(&input), 36);
    }
}
