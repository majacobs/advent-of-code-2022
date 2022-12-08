use crate::harness::run;
use std::time::Duration;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn run_all() {
    println!("==== 2015 ====");
    let mut runtime = Duration::default();
    runtime += run(2015, 1, day01::Solution);
    runtime += run(2015, 2, day02::Solution);
    runtime += run(2015, 3, day03::Solution);
    runtime += run(2015, 4, day04::Solution);

    println!();
    println!("Total runtime: {} ms", runtime.as_millis());
}
