use crate::harness::run;
use std::time::Duration;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn run_all() {
    println!("==== 2022 ====");
    let mut runtime = Duration::default();
    runtime += run(2022, 1, day01::Solution);
    runtime += run(2022, 2, day02::Solution);
    runtime += run(2022, 3, day03::Solution);
    runtime += run(2022, 4, day04::Solution);
    runtime += run(2022, 5, day05::Solution);
    runtime += run(2022, 6, day06::Solution);
    runtime += run(2022, 7, day07::Solution);
    runtime += run(2022, 8, day08::Solution);
    runtime += run(2022, 9, day09::Solution);
    runtime += run(2022, 10, day10::Solution);

    println!();
    println!("Total runtime: {} ms", runtime.as_millis());
}
