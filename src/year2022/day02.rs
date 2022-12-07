use crate::harness::Harness;

pub struct Solution;

impl Harness for Solution {
    type Parsed = Vec<(char, char)>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse(&self, raw_input: String) -> Self::Parsed {
        raw_input
            .lines()
            .map(|s| {
                let mut c = s.chars();
                let c1 = c.next().unwrap();
                c.next();
                let c2 = c.next().unwrap();
                (c1, c2)
            })
            .collect()
    }

    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut score = 0;
        for i in input {
            let opponent = Play::from(i.0);
            let mine = Play::from(i.1);
            score += mine.points() + mine.compare(&opponent).points()
        }
        score
    }

    fn part2(&self, input: &Self::Parsed) -> Self::Part1Output {
        let mut score = 0;
        for i in input {
            let opponent = Play::from(i.0);
            let outcome = Outcome::from(i.1);
            score += outcome.foo(opponent).points() + outcome.points()
        }
        score
    }
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn compare(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissors) => Outcome::Win,

            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissors) => Outcome::Loss,

            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }

    fn points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn foo(&self, opponent: Play) -> Play {
        match (self, opponent) {
            (Self::Win, Play::Rock) => Play::Paper,
            (Self::Win, Play::Paper) => Play::Scissors,
            (Self::Win, Play::Scissors) => Play::Rock,

            (Self::Draw, p) => p,

            (Self::Loss, Play::Rock) => Play::Scissors,
            (Self::Loss, Play::Paper) => Play::Rock,
            (Self::Loss, Play::Scissors) => Play::Paper,
        }
    }
}
