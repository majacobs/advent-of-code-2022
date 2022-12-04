use std::fs;

pub fn run() {
    let input = read();
    println!("Day 4");
    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn read() -> Vec<((u32, u32), (u32, u32))> {
    fs::read_to_string("input/day04")
        .expect("Unable to read input")
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                let mut ranges = s.split([',', '-']);
                Some((
                    (
                        ranges.next().unwrap().parse().unwrap(),
                        ranges.next().unwrap().parse().unwrap(),
                    ),
                    (
                        ranges.next().unwrap().parse().unwrap(),
                        ranges.next().unwrap().parse().unwrap(),
                    ),
                ))
            }
        })
        .collect()
}

fn part1(input: &[((u32, u32), (u32, u32))]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.0 >= b.0 && a.1 <= b.1 || b.0 >= a.0 && b.1 <= a.1)
        .count()
}

fn part2(input: &[((u32, u32), (u32, u32))]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.0 <= b.1 && a.1 >= b.0)
        .count()
}
