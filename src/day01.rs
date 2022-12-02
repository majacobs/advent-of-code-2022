use std::cmp::Reverse;
use std::fs;

pub fn run() {
    let input = read();
    println!("Day 1");
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn read() -> Vec<Option<i32>> {
    fs::read_to_string("input/day01")
        .expect("Unable to read input")
        .split('\n')
        .map(|s| s.parse().ok())
        .collect()
}

fn part1(input: &[Option<i32>]) -> i32 {
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

fn part2(input: &[Option<i32>]) -> i32 {
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
