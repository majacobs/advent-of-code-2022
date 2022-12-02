use std::fs;

pub fn run() {
    let input = read();
    println!("Day 2");
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn read() -> Vec<[char; 2]> {
    fs::read_to_string("input/day02")
        .expect("Unable to read input")
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                return None;
            }

            let mut c = s.chars();
            let c1 = c.next().unwrap();
            c.next();
            let c2 = c.next().unwrap();
            Some([c1, c2])
        })
        .collect()
}

fn part1(input: &[[char; 2]]) -> u32 {
    let mut score = 0;
    for i in input {
        let opponent = Play::from(i[0]);
        let mine = Play::from(i[1]);
        score += mine.points() + mine.compare(&opponent).points()
    }
    score
}

fn part2(input: &[[char; 2]]) -> u32 {
    let mut score = 0;
    for i in input {
        let opponent = Play::from(i[0]);
        let outcome = Outcome::from(i[1]);
        score += outcome.foo(opponent).points() + outcome.points()
    }
    score
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
