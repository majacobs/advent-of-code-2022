use std::time::{Duration, Instant};

pub fn run(year: u32, day: u32, solution: impl Harness) -> Duration {
    let path = format!("input/{}/day{:02}", year, day);
    let Ok(raw) = std::fs::read_to_string(path) else {
        println!("Skipping {} day {}, input file missing.", year, day);
        return Duration::default();
    };
    let mut runtime = Duration::default();

    println!("Day {}", day);
    let input = solution.parse(raw);
    runtime += measure(|| solution.part1(&input), 1);
    runtime += measure(|| solution.part2(&input), 2);
    runtime
}

pub trait Harness {
    type Parsed;
    type Part1Output: std::fmt::Display;
    type Part2Output: std::fmt::Display;

    fn parse(&self, raw_input: String) -> Self::Parsed;
    fn part1(&self, input: &Self::Parsed) -> Self::Part1Output;
    fn part2(&self, input: &Self::Parsed) -> Self::Part2Output;
}

fn measure<F, O>(f: F, part: u32) -> Duration
where
    F: FnOnce() -> O,
    O: std::fmt::Display,
{
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    println!("  Part {}: {}", part, result);

    let runtime = end - start;
    if runtime.as_millis() > 0 {
        println!("    Runtime: {} ms", runtime.as_millis());
    } else {
        println!("    Runtime: {} \u{00b5}s", runtime.as_micros());
    }

    runtime
}
