use std::fs;

pub fn run() {
    let input = read();
    println!("Day 6");
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn read() -> String {
    fs::read_to_string("input/day06").expect("Unable to read input")
}

fn part1(input: &str) -> usize {
    const SIZE: usize = 4;
    let window_start = input
        .chars()
        .collect::<Vec<char>>()
        .windows(SIZE)
        .enumerate()
        .find(|(_, chars)| all_unique(chars))
        .unwrap()
        .0;
    window_start + SIZE
}

fn part2(input: &str) -> usize {
    const SIZE: usize = 14;
    let window_start = input
        .chars()
        .collect::<Vec<char>>()
        .windows(SIZE)
        .enumerate()
        .find(|(_, chars)| all_unique(chars))
        .unwrap()
        .0;
    window_start + SIZE
}

fn all_unique(chars: &[char]) -> bool {
    let size = chars.len();
    for i in 0..size {
        for j in (i + 1)..size {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}
