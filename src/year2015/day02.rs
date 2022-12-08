use crate::harness::Harness;
use std::str::FromStr;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<Box>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        input.iter().map(|b| b.coverage()).sum()
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output {
        input.iter().map(|b| b.volume() + b.min_girth()).sum()
    }
}

pub struct Box {
    length: u32,
    width: u32,
    height: u32,
}

impl Box {
    fn coverage(&self) -> u32 {
        let a = self.length * self.width;
        let b = self.width * self.height;
        let c = self.height * self.length;
        let slack = a.min(b).min(c);
        2 * (a + b + c) + slack
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn min_girth(&self) -> u32 {
        let exclude = self.length.max(self.width).max(self.height);
        2 * (self.length + self.width + self.height - exclude)
    }
}

impl FromStr for Box {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, 'x');
        let length = parts.next().unwrap().parse().unwrap();
        let width = parts.next().unwrap().parse().unwrap();
        let height = parts.next().unwrap().parse().unwrap();
        Ok(Self {
            length,
            width,
            height,
        })
    }
}
